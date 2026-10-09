// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! A desktop in one process: the window manager's table, z stack and focus,
//! the compositor's scene and damage, the input router's press grab, and an
//! app_skeleton frame per window. Each method carries one step of the pointer
//! path the way the capsules carry it over IPC, calling the real code at every
//! hop; the glue between the hops is the order the capsules call them in.

use nonos_toolkit::decorations::accessory_rect;

use super::drag::{self, DragState, PointerAction};
use super::press_part::Route;
use crate::damage::{DamageAccumulator, Rect as Damage};
use crate::focus::{press_focus, topmost_hit_at, FocusModel};
use crate::geometry::resize::resized;
use crate::geometry::{clamp_to_display, Rect};
use crate::input::{InputEvent, InputKind};
use crate::press::Press;
use crate::scene::{Layer, SceneTable};
use crate::scene_raise::raise_by_pid;
use crate::scene_submit::submit_layer;
use crate::window::full_screen::{covers_dock_at, maximize, MAXIMIZE_FLAG_FULL_SCREEN};
use crate::window::{Kind, Visibility, Window, WindowTable};
use crate::z_order::{bottom_up, raise, ZStack};

/// app_skeleton's layer band for application windows.
const APP_LAYER_Z: u32 = 2;
/// The desktop shell's desk band, under every application window.
const SHELL_LAYER_Z: u32 = 1;
/// The desktop shell's chrome band, over every application window
/// (capsule_desktop_shell setup/prime/register.rs CHROME_Z).
const SHELL_CHROME_Z: u32 = 3_000_000;
/// The shell's dock window (state/chrome.rs TASKBAR_WINDOW_ID).
pub const DOCK_WINDOW_ID: u32 = 0x5442_4152;
/// The ring of shadow the shell draws round the dock at one to one
/// (render/panel.rs shadow_panel, px(3)).
const DOCK_SHADOW: u32 = 3;
/// What the chrome surface holds at a pixel: nothing (transparent, the
/// windows under it show), the dock's panel, or its shadow.
const CLEAR: u8 = 0;
const PANEL: u8 = 1;
const SHADOW: u8 = 2;

/// One window's client side: where it believes it is, and its frame state.
pub struct App {
    pub pid: u32,
    pub window_id: u32,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub accessory_w: u32,
    pub drag: DragState,
    pub accessory_events: u32,
    pub moves: u32,
    /// The size the window opened at, which min_size holds it to.
    pub opened: (u32, u32),
    /// The rect green restores, while the window is full screen.
    pub saved: Option<(u32, u32, u32, u32)>,
}

pub struct Desk {
    pub width: u32,
    pub height: u32,
    pub windows: WindowTable,
    pub z: ZStack,
    pub focus: FocusModel,
    pub scene: SceneTable,
    pub damage: DamageAccumulator,
    pub screen: Vec<u32>,
    pub press: Option<Press>,
    pub apps: Vec<App>,
    pub shell_presses: u32,
    /// The desktop shell's pid once it is up, else 0. A press on no window
    /// goes to it.
    pub shell: u32,
    /// The compositor takes no focus_set: each one the window manager sends
    /// is lost (the window manager owes it a restack, state/restack.rs).
    pub compositor_lost: bool,
    /// The shell's chrome surface, a full-screen layer over every window:
    /// what it holds at each pixel (CLEAR, PANEL, SHADOW). The compositor
    /// shows the window under it where it is clear.
    pub chrome: Vec<u8>,
    /// The shell's dock: its rule (state/taskbar, the real code) and rect.
    pub taskbar: crate::taskbar::TaskbarState,
    pub dock: (u32, u32, u32, u32),
    /// Rects the shell committed as damage for its dock.
    pub dock_commits: Vec<Damage>,
}

impl Desk {
    pub fn new(width: u32, height: u32) -> Desk {
        Desk {
            width,
            height,
            windows: WindowTable::new(),
            z: ZStack::new(),
            focus: FocusModel::new(),
            scene: SceneTable::new(),
            damage: DamageAccumulator::new(),
            screen: vec![0; (width * height) as usize],
            press: None,
            apps: Vec::new(),
            shell_presses: 0,
            shell: 0,
            compositor_lost: false,
            chrome: vec![CLEAR; (width * height) as usize],
            taskbar: crate::taskbar::new_taskbar_state(),
            dock: (0, 0, 0, 0),
            dock_commits: Vec::new(),
        }
    }

    /// The desktop shell as it primes: its desk, a full-screen layer in the
    /// band under application windows (DESK_Z); its chrome in the band over
    /// them (CHROME_Z), modelled here by the one part of it that is opaque
    /// while no menu is open, the dock; and a popup window over the dock so
    /// the window manager knows the dock is there.
    pub fn open_shell(&mut self, pid: u32, dock: (u32, u32, u32, u32)) {
        self.shell = pid;
        self.submit_band(pid, 0, 0, self.width, self.height, SHELL_LAYER_Z);
        self.submit_band(pid, 0, 0, self.width, self.height, SHELL_CHROME_Z);
        self.dock = dock;
        self.paint_dock();
        let window = Window {
            owner_pid: pid,
            window_id: DOCK_WINDOW_ID,
            rect: Rect { x: dock.0, y: dock.1, width: dock.2, height: dock.3 },
            kind: Kind::Popup,
            visibility: Visibility::Visible,
            z: self.z.allocate(),
            in_use: true,
            full_screen: false,
        };
        self.windows.insert(window).expect("room for the dock");
        self.frame();
    }

    /// window_open (a new z, focus for a normal window), then the app's first
    /// scene submit.
    pub fn open(&mut self, pid: u32, r: (u32, u32, u32, u32), accessory_w: u32) {
        let window_id = 0x5749_4E00 | pid;
        let rect = clamp_to_display(
            Rect { x: r.0, y: r.1, width: r.2, height: r.3 },
            self.width,
            self.height,
        );
        let window = Window {
            owner_pid: pid,
            window_id,
            rect,
            kind: Kind::Normal,
            visibility: Visibility::Visible,
            z: self.z.allocate(),
            in_use: true,
            full_screen: false,
        };
        self.windows.insert(window).expect("room for the window");
        self.focus.set(pid, window_id);
        self.submit(pid, rect.x, rect.y, rect.width, rect.height);
        self.apps.push(App {
            pid,
            window_id,
            x: rect.x,
            y: rect.y,
            w: rect.width,
            h: rect.height,
            accessory_w,
            drag: DragState::new(),
            accessory_events: 0,
            moves: 0,
            opened: (rect.width, rect.height),
            saved: None,
        });
        self.frame();
    }

    fn submit(&mut self, pid: u32, x: u32, y: u32, w: u32, h: u32) {
        self.submit_band(pid, x, y, w, h, APP_LAYER_Z);
    }

    fn submit_band(&mut self, pid: u32, x: u32, y: u32, w: u32, h: u32, z: u32) {
        let layer = Layer {
            owner_pid: pid,
            surface_handle: pid as u64,
            x,
            y,
            width: w,
            height: h,
            z,
            stack: 0,
            in_use: true,
            miss_count: 0,
        };
        for r in
            submit_layer(&mut self.scene, layer).expect("room for the layer").into_iter().flatten()
        {
            self.damage.accumulate(r);
        }
    }

    /// A button press at screen (x, y), as route_pointer handles it.
    pub fn press(&mut self, x: u32, y: u32) {
        self.press = None;
        // No window, or the shell's own: route_to_shell, no focus change.
        let hit =
            topmost_hit_at(&self.windows, x, y, self.shell).filter(|h| h.owner_pid != self.shell);
        let Some(hit) = hit else {
            self.shell_presses += 1;
            return;
        };
        // route_to_window: route_focus first (the window manager focuses and
        // raises, and pushes focus_set when that restacked anything) ...
        let pressed = press_focus(
            &mut self.windows,
            &mut self.z,
            &mut self.focus,
            hit.owner_pid,
            hit.window_id,
        )
        .expect("the hit window takes focus");
        if pressed.restacked() && !self.compositor_lost {
            if let Some(r) = raise_by_pid(&mut self.scene, hit.owner_pid) {
                self.damage.accumulate(r);
            }
        }
        // ... then the press grab, then the press in window coordinates.
        self.press = Some(Press::arm(hit.owner_pid, x, y, hit.local_x, hit.local_y));
        let ev = event(InputKind::ButtonDown, hit.local_x as i32, hit.local_y as i32);
        self.deliver(hit.owner_pid, ev);
        self.frame();
    }

    /// Pointer motion to screen (x, y): through the press grab while a button
    /// is down, else to the window under the pointer in its own coordinates.
    pub fn motion(&mut self, x: u32, y: u32) {
        self.route(InputKind::PointerAbs, x, y);
        if self.shell != 0 {
            self.shell_motion(y);
        }
    }

    pub fn release(&mut self, x: u32, y: u32) {
        self.route(InputKind::ButtonUp, x, y);
        self.press = None;
    }

    fn route(&mut self, kind: InputKind, x: u32, y: u32) {
        if let Some(p) = self.press {
            let (lx, ly) = p.local(x, y);
            self.deliver(p.pid, event(kind, lx, ly));
        } else if let Some(hit) = topmost_hit_at(&self.windows, x, y, self.shell) {
            let ev = event(kind, hit.local_x as i32, hit.local_y as i32);
            self.deliver(hit.owner_pid, ev);
        }
        self.frame();
    }

    /// The window's drain: click to focus on a press, the press routing, the
    /// drag, then a move applied as apply_move applies it.
    fn deliver(&mut self, pid: u32, ev: InputEvent) {
        let Some(i) = self.apps.iter().position(|a| a.pid == pid) else { return };
        if ev.kind == InputKind::ButtonDown {
            // click_focus: the app's own raise and focus, after the router's.
            let wid = self.apps[i].window_id;
            if raise(&mut self.windows, &mut self.z, pid, wid) == Some(true)
                && !self.compositor_lost
            {
                if let Some(r) = raise_by_pid(&mut self.scene, pid) {
                    self.damage.accumulate(r);
                }
            }
            self.focus.set(pid, wid);
        }
        let app = &mut self.apps[i];
        let acc = accessory_rect(app.w, app.h, false, app.accessory_w);
        let ev = match app.drag.press.route(ev, acc) {
            Route::Accessory(_) => {
                app.accessory_events += 1;
                return;
            }
            Route::Window(ev) => ev,
        };
        let action = drag::handle(&mut app.drag, app.w, app.h, app.x, app.y, false, &ev);
        match action {
            PointerAction::MoveTo(nx, ny) => self.apply_move(i, nx, ny),
            PointerAction::ResizeTo(nw, nh) => self.apply_resize(i, nw, nh),
            _ => {}
        }
    }

    /// apply_resize: the size settled against the opening size and the room
    /// the display leaves right of and below the origin, the new surface
    /// submitted at the same origin, then window_resize as the window manager
    /// takes it.
    pub fn apply_resize(&mut self, i: usize, w: u32, h: u32) {
        let (x, y, pid, wid) =
            (self.apps[i].x, self.apps[i].y, self.apps[i].pid, self.apps[i].window_id);
        let bound = Some(super::min_size::room((self.width, self.height), (x, y)));
        let opened = self.apps[i].opened;
        let (w, h) = super::min_size::settle((w, h), opened, bound);
        if (w, h) == (self.apps[i].w, self.apps[i].h) {
            return;
        }
        // reopen_surface: the new surface at the same origin, in the layer the
        // window has (it was pressed, so it is on top of its band already).
        self.submit(pid, x, y, w, h);
        let wm = self.windows.find_mut(pid, wid).expect("open");
        wm.rect = resized(wm.rect, w, h, self.width, self.height);
        self.apps[i].w = w;
        self.apps[i].h = h;
        self.frame();
    }

    /// The green button as maximize::toggle presses it: the surface
    /// reopened at chrome::full_screen (or the saved rect), submitted, then
    /// window_maximize with the full-screen flag (or without it).
    pub fn green(&mut self, pid: u32) {
        self.wm_change(pid, |d, _| d.toggle_full_screen(pid));
    }

    fn toggle_full_screen(&mut self, pid: u32) {
        let i = self.apps.iter().position(|a| a.pid == pid).expect("open");
        let wid = self.apps[i].window_id;
        let (rect, flags) = match self.apps[i].saved.take() {
            Some(saved) => (saved, 0),
            None => {
                let a = &self.apps[i];
                self.apps[i].saved = Some((a.x, a.y, a.w, a.h));
                (super::chrome::full_screen(self.width, self.height), MAXIMIZE_FLAG_FULL_SCREEN)
            }
        };
        let (x, y, w, h) = rect;
        self.submit(pid, x, y, w, h);
        let (dw, dh) = (self.width, self.height);
        let wm = self.windows.find_mut(pid, wid).expect("open");
        maximize(wm, Rect { x, y, width: w, height: h }, flags, dw, dh);
        if raise(&mut self.windows, &mut self.z, pid, wid) == Some(true) {
            if let Some(r) = raise_by_pid(&mut self.scene, pid) {
                self.damage.accumulate(r);
            }
        }
        let a = &mut self.apps[i];
        (a.x, a.y, a.w, a.h) = (x, y, w, h);
        self.frame();
    }

    fn apply_move(&mut self, i: usize, to_x: u32, to_y: u32) {
        let (w, h, pid, wid) =
            (self.apps[i].w, self.apps[i].h, self.apps[i].pid, self.apps[i].window_id);
        let nx = to_x.min(self.width.saturating_sub(w));
        let ny = to_y.min(self.height.saturating_sub(h));
        if (nx, ny) == (self.apps[i].x, self.apps[i].y) {
            return;
        }
        self.submit(pid, nx, ny, w, h);
        let wm = self.windows.find_mut(pid, wid).expect("open");
        wm.rect =
            clamp_to_display(Rect { x: nx, y: ny, width: w, height: h }, self.width, self.height);
        self.apps[i].x = nx;
        self.apps[i].y = ny;
        self.apps[i].moves += 1;
    }

    /// The window manager's owed restack (server/tell_compositor.rs resync):
    /// every process with a window showing lifted, bottom of the stack first.
    pub fn restack(&mut self) {
        let mut lifted = Vec::new();
        bottom_up(&self.windows, |pid| {
            lifted.push(pid);
            true
        });
        for pid in lifted {
            if let Some(r) = raise_by_pid(&mut self.scene, pid) {
                self.damage.accumulate(r);
            }
        }
        self.frame();
    }

    /// One composed frame: every damaged rectangle repainted, nothing else.
    fn frame(&mut self) {
        while let Some(r) = self.damage.drain() {
            self.paint(r);
        }
    }

    fn paint(&mut self, clip: Damage) {
        let (x1, y1) =
            ((clip.x + clip.width).min(self.width), (clip.y + clip.height).min(self.height));
        let (layers, n) = self.scene.z_sorted_snapshot();
        for y in clip.y..y1 {
            for x in clip.x..x1 {
                let i = (y * self.width + x) as usize;
                let top = layers[..n]
                    .iter()
                    .rfind(|l| {
                        let inside =
                            x >= l.x && x < l.x + l.width && y >= l.y && y < l.y + l.height;
                        let clear = l.z == SHELL_CHROME_Z && self.chrome[i] == CLEAR;
                        inside && !clear
                    })
                    .map_or(0, |l| l.owner_pid);
                self.screen[(y * self.width + x) as usize] = top;
            }
        }
    }

    /// What the screen shows at a point.
    pub fn shown_at(&self, x: u32, y: u32) -> u32 {
        self.screen[(y * self.width + x) as usize]
    }

    /// Who a press at a point would go to: the window hit, or the shell (0
    /// before it is up) where no window is.
    pub fn hit(&self, x: u32, y: u32) -> u32 {
        topmost_hit_at(&self.windows, x, y, self.shell).map_or(self.shell, |h| h.owner_pid)
    }

    pub fn focused(&self) -> u32 {
        self.focus.current().map_or(0, |f| f.owner_pid)
    }

    pub fn app(&self, pid: u32) -> &App {
        self.apps.iter().find(|a| a.pid == pid).expect("open")
    }

    /// Every pixel the screen shows equals a full recomposite, and is the
    /// window a press there goes to (windows are opaque here, margins too).
    pub fn assert_consistent(&mut self, what: &str) {
        let shown = self.screen.clone();
        let all = Damage { x: 0, y: 0, width: self.width, height: self.height };
        self.paint(all);
        for y in (0..self.height).step_by(3) {
            for x in (0..self.width).step_by(3) {
                let i = (y * self.width + x) as usize;
                assert_eq!(shown[i], self.screen[i], "{what}: stale pixel at ({x}, {y})");
                // The dock's shadow is drawn but takes no press: the dock's
                // window is its panel.
                if self.chrome[i] == SHADOW && self.screen[i] == self.shell {
                    continue;
                }
                assert_eq!(
                    self.screen[i],
                    self.hit(x, y),
                    "{what}: ({x}, {y}) shows one window, a press goes to another"
                );
            }
        }
    }
}

impl Desk {
    /// The shell paints its chrome: cleared, and the dock with its shadow
    /// when its rule shows it (render/chrome.rs paint_chrome). Nothing is
    /// committed here.
    fn paint_dock(&mut self) {
        self.chrome.fill(CLEAR);
        if !self.taskbar.visible {
            return;
        }
        let (dx, dy, dw, dh) = self.dock;
        let (x0, y0) = (dx.saturating_sub(DOCK_SHADOW), dy.saturating_sub(DOCK_SHADOW));
        let x1 = (dx + dw + DOCK_SHADOW).min(self.width);
        let y1 = (dy + dh + DOCK_SHADOW).min(self.height);
        for y in y0..y1 {
            for x in x0..x1 {
                let panel = x >= dx && x < dx + dw && y >= dy && y < dy + dh;
                self.chrome[(y * self.width + x) as usize] = if panel { PANEL } else { SHADOW };
            }
        }
    }

    /// The dock's whole area as render/layout.rs dock_area_rect has it at one
    /// to one: panel and shadow and a pixel more, down to the bottom edge.
    pub fn dock_area(&self) -> Damage {
        let (dx, dy, dw, _) = self.dock;
        let m = DOCK_SHADOW + 1;
        let (x, y) = (dx.saturating_sub(m), dy.saturating_sub(m));
        let width = (dw + 2 * m).min(self.width - x);
        Damage { x, y, width, height: self.height - y }
    }

    /// server/dock_sync.rs: repaint and commit the dock's area, and open or
    /// close its window, as the rule asks.
    pub fn shell_sync(&mut self) {
        let work = crate::taskbar::dock_work(&self.taskbar);
        if work.paint {
            self.paint_dock();
            let area = self.dock_area();
            self.damage.accumulate(area);
            self.dock_commits.push(area);
            self.taskbar.drawn = self.taskbar.visible;
        }
        match work.window {
            Some(true) => {
                let (x, y, width, height) = self.dock;
                let window = Window {
                    owner_pid: self.shell,
                    window_id: DOCK_WINDOW_ID,
                    rect: Rect { x, y, width, height },
                    kind: Kind::Popup,
                    visibility: Visibility::Visible,
                    z: self.z.allocate(),
                    in_use: true,
                    full_screen: false,
                };
                self.windows.insert(window).expect("room for the dock");
                self.taskbar.window_open = true;
            }
            Some(false) => {
                let _ = self.windows.remove(self.shell, DOCK_WINDOW_ID);
                self.taskbar.window_open = false;
            }
            None => {}
        }
        self.frame();
    }

    /// A window manager change to `pid`'s window, then the full-screen
    /// notification the window manager sends when whether it covers the
    /// dock's band changed (server/full_screen_notify.rs), as the shell
    /// takes it (server/wm_notify.rs), then the shell's sync.
    fn wm_change(&mut self, pid: u32, change: impl FnOnce(&mut Self, u32)) {
        let wid = 0x5749_4E00 | pid;
        let was = covers_dock_at(&self.windows, pid, wid);
        change(self, wid);
        let now = covers_dock_at(&self.windows, pid, wid);
        if now != was && self.shell != 0 {
            crate::taskbar::set_full_screen(&mut self.taskbar, pid, wid, now);
        }
        self.shell_sync();
    }

    /// The window's minimise button: window_minimize, then its layer removed.
    pub fn minimize(&mut self, pid: u32) {
        self.wm_change(pid, |d, wid| {
            d.windows.find_mut(pid, wid).expect("open").visibility = Visibility::Minimized;
            d.drop_layer(pid);
        });
    }

    /// The dock raising a minimised window: window_restore, then the app's
    /// layer submitted again where it was.
    pub fn restore_window(&mut self, pid: u32) {
        self.wm_change(pid, |d, wid| {
            d.windows.find_mut(pid, wid).expect("open").visibility = Visibility::Visible;
            let _ = raise(&mut d.windows, &mut d.z, pid, wid);
            let a = d.app(pid);
            let (x, y, w, h) = (a.x, a.y, a.w, a.h);
            d.submit(pid, x, y, w, h);
        });
    }

    /// The window's close button: its layer gone, window_close, and the
    /// closed event the shell takes as the end of its full screen.
    pub fn close(&mut self, pid: u32) {
        let wid = 0x5749_4E00 | pid;
        self.drop_layer(pid);
        let _ = self.windows.remove(pid, wid);
        self.apps.retain(|a| a.pid != pid);
        if self.shell != 0 {
            crate::taskbar::set_full_screen(&mut self.taskbar, pid, wid, false);
        }
        self.shell_sync();
    }

    fn drop_layer(&mut self, pid: u32) {
        let a = self.app(pid);
        let r = Damage { x: a.x, y: a.y, width: a.w, height: a.h };
        self.scene.drop_by_pid(pid);
        self.damage.accumulate(r);
        self.frame();
    }

    /// Pointer motion as the input router mirrors it to the shell, wherever
    /// it is (mirror_shell_pointer), taken by hover_reveal.
    pub fn shell_motion(&mut self, y: u32) {
        let top = self.dock_area().y;
        crate::taskbar::dock_pointer(&mut self.taskbar, y, self.height, top);
        self.shell_sync();
    }
}

pub fn event(kind: InputKind, x: i32, y: i32) -> InputEvent {
    InputEvent { kind, flags: 0, code: 0, x, y, delta_x: 0, delta_y: 0, timestamp_ns: 0 }
}
