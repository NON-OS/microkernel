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

//! Click to focus against the real window table, z stack and hit test. A press
//! the router reports on a window must focus it and put it on top in the same
//! step, so the very next hit test (the drag's first motion, or a second
//! click) finds the window the user is looking at.

use crate::focus::press::{Pressed, Refused};
use crate::focus::{press_focus, topmost_hit_at, FocusModel};
use crate::geometry::Rect;
use crate::window::{Kind, Visibility, Window, WindowTable};
use crate::z_order::{raise, ZStack};

const TERMINAL: (u32, u32) = (40, 0x5445_524D);
const EDITOR: (u32, u32) = (41, 0x5445_4458);

struct Desk {
    windows: WindowTable,
    z: ZStack,
    focus: FocusModel,
}

impl Desk {
    fn new() -> Desk {
        Desk { windows: WindowTable::new(), z: ZStack::new(), focus: FocusModel::new() }
    }

    /// Open a window the way window_open does: a fresh z, on top.
    fn open(&mut self, who: (u32, u32), r: (u32, u32, u32, u32), kind: Kind) {
        let window = Window {
            owner_pid: who.0,
            window_id: who.1,
            rect: Rect { x: r.0, y: r.1, width: r.2, height: r.3 },
            kind,
            visibility: Visibility::Visible,
            z: self.z.allocate(),
            in_use: true,
            full_screen: false,
        };
        self.windows.insert(window).expect("room in the table");
        if kind == Kind::Normal {
            self.focus.set(who.0, who.1);
        }
    }

    fn press(&mut self, x: u32, y: u32) -> Option<((u32, u32), Result<Pressed, Refused>)> {
        let hit = topmost_hit_at(&self.windows, x, y, 0)?;
        let who = (hit.owner_pid, hit.window_id);
        Some((who, press_focus(&mut self.windows, &mut self.z, &mut self.focus, who.0, who.1)))
    }

    fn hit(&self, x: u32, y: u32) -> Option<(u32, u32)> {
        topmost_hit_at(&self.windows, x, y, 0).map(|h| (h.owner_pid, h.window_id))
    }

    fn focused(&self) -> Option<(u32, u32)> {
        self.focus.current().map(|f| (f.owner_pid, f.window_id))
    }
}

/// The reported bug, as the stack sees it. The terminal opens, then the editor
/// opens over most of it. A press on the part of the terminal still showing
/// focuses it, and the compositor draws it on top straight away. The very next
/// hit test, at a point where the two overlap (the terminal's title bar the
/// user now reaches for), must find the terminal. Before, the press moved focus
/// only, the editor kept the higher z, and that press went to the editor.
#[test]
fn a_press_on_a_covered_window_raises_it_before_the_next_hit_test() {
    let mut desk = Desk::new();
    desk.open(TERMINAL, (100, 100, 400, 300), Kind::Normal);
    desk.open(EDITOR, (160, 60, 600, 500), Kind::Normal);
    assert_eq!(desk.hit(300, 120), Some(EDITOR), "the editor covers the terminal's title bar");

    let (who, pressed) = desk.press(120, 300).expect("the terminal shows at its left edge");
    assert_eq!(who, TERMINAL);
    assert_eq!(pressed, Ok(Pressed { focus_changed: true, raised: true }));
    assert_eq!(desk.focused(), Some(TERMINAL));
    assert_eq!(desk.hit(300, 120), Some(TERMINAL), "the title bar now belongs to the terminal");
}

#[test]
fn a_press_on_the_window_on_top_restacks_nothing() {
    let mut desk = Desk::new();
    desk.open(TERMINAL, (100, 100, 400, 300), Kind::Normal);
    desk.open(EDITOR, (160, 60, 600, 500), Kind::Normal);
    let (who, pressed) = desk.press(300, 120).expect("on the editor");
    assert_eq!(who, EDITOR);
    let pressed = pressed.expect("focusable");
    assert!(!pressed.restacked(), "already focused and on top: nothing to tell the compositor");
}

#[test]
fn a_press_on_a_focused_window_under_a_dialog_still_raises_it() {
    let mut desk = Desk::new();
    desk.open(EDITOR, (0, 0, 400, 300), Kind::Normal);
    desk.open((50, 1), (100, 100, 200, 100), Kind::Dialog);
    assert_eq!(desk.focused(), Some(EDITOR), "a dialog opening does not take focus");
    let pressed = press_focus(&mut desk.windows, &mut desk.z, &mut desk.focus, EDITOR.0, EDITOR.1);
    assert_eq!(pressed, Ok(Pressed { focus_changed: false, raised: true }));
    assert_eq!(desk.hit(150, 150), Some(EDITOR));
}

#[test]
fn a_press_on_a_tooltip_changes_nothing() {
    let mut desk = Desk::new();
    desk.open(EDITOR, (0, 0, 400, 300), Kind::Normal);
    desk.open((50, 2), (10, 10, 50, 20), Kind::Tooltip);
    let before = desk.windows.find(50, 2).map(|w| w.z);
    let r = press_focus(&mut desk.windows, &mut desk.z, &mut desk.focus, 50, 2);
    assert_eq!(r, Err(Refused::NotFocusable));
    assert_eq!(desk.focused(), Some(EDITOR));
    assert_eq!(desk.windows.find(50, 2).map(|w| w.z), before);
}

#[test]
fn a_press_on_a_window_that_closed_meanwhile_is_refused() {
    let mut desk = Desk::new();
    desk.open(EDITOR, (0, 0, 400, 300), Kind::Normal);
    let r = press_focus(&mut desk.windows, &mut desk.z, &mut desk.focus, 77, 1);
    assert_eq!(r, Err(Refused::NoWindow));
    assert_eq!(desk.focused(), Some(EDITOR));
}

#[test]
fn raise_reports_whether_the_order_moved() {
    let mut desk = Desk::new();
    desk.open(TERMINAL, (0, 0, 10, 10), Kind::Normal);
    desk.open(EDITOR, (0, 0, 10, 10), Kind::Normal);
    assert_eq!(raise(&mut desk.windows, &mut desk.z, EDITOR.0, EDITOR.1), Some(false));
    assert_eq!(raise(&mut desk.windows, &mut desk.z, TERMINAL.0, TERMINAL.1), Some(true));
    assert_eq!(raise(&mut desk.windows, &mut desk.z, TERMINAL.0, TERMINAL.1), Some(false));
    assert_eq!(raise(&mut desk.windows, &mut desk.z, 9, 9), None);
    assert_eq!(desk.hit(5, 5), Some(TERMINAL));
}

/// window_open for a window the table still holds (its owner opened it
/// again without a close reaching the window manager) raises it as it
/// focuses it: the compositor puts the layer the owner submits next on top,
/// and the stack must agree.
#[test]
fn a_window_opened_again_goes_back_on_top() {
    let mut desk = Desk::new();
    desk.open(TERMINAL, (0, 0, 100, 100), Kind::Normal);
    desk.open(EDITOR, (50, 50, 100, 100), Kind::Normal);
    assert_eq!(desk.hit(75, 75), Some(EDITOR));
    assert_eq!(raise(&mut desk.windows, &mut desk.z, TERMINAL.0, TERMINAL.1), Some(true));
    desk.focus.set(TERMINAL.0, TERMINAL.1);
    assert_eq!(desk.hit(75, 75), Some(TERMINAL));
    assert_eq!(desk.focused(), Some(TERMINAL));
}

#[test]
fn the_hit_test_skips_minimized_windows_and_tooltips() {
    let mut desk = Desk::new();
    desk.open(TERMINAL, (0, 0, 100, 100), Kind::Normal);
    desk.open(EDITOR, (0, 0, 100, 100), Kind::Normal);
    desk.open((50, 3), (0, 0, 100, 100), Kind::Tooltip);
    assert_eq!(desk.hit(50, 50), Some(EDITOR), "a tooltip takes no click");
    desk.windows.find_mut(EDITOR.0, EDITOR.1).expect("open").visibility = Visibility::Minimized;
    assert_eq!(desk.hit(50, 50), Some(TERMINAL), "a minimized window takes no click");
    assert_eq!(desk.hit(100, 50), None, "the right edge is outside");
}

#[test]
fn the_hit_reports_where_in_the_window_the_press_landed() {
    let mut desk = Desk::new();
    desk.open(TERMINAL, (100, 60, 400, 300), Kind::Normal);
    let h = topmost_hit_at(&desk.windows, 130, 75, 0).expect("inside");
    assert_eq!((h.local_x, h.local_y), (30, 15));
    assert_eq!((h.win_x, h.win_y, h.win_w, h.win_h), (100, 60, 400, 300));
}

/// Random desks, random presses: whatever window a press lands on is focused
/// and on top afterwards, so a second press at the same point lands on it
/// again, and every point it covers now hits it.
#[test]
fn whatever_a_press_lands_on_is_on_top_after_it() {
    let mut seed = 0x2545_F491_4F6C_DD1Du64;
    let mut below = |n: u32| {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        (seed % n as u64) as u32
    };
    for _ in 0..200 {
        let mut desk = Desk::new();
        let count = 2 + below(5);
        for i in 0..count {
            let (w, h) = (40 + below(200), 40 + below(150));
            desk.open((10 + i, 1), (below(400), below(300), w, h), Kind::Normal);
        }
        for _ in 0..20 {
            let (x, y) = (below(640), below(480));
            let Some((who, pressed)) = desk.press(x, y) else { continue };
            assert!(pressed.is_ok());
            assert_eq!(desk.focused(), Some(who));
            assert_eq!(desk.hit(x, y), Some(who), "a second press lands on the same window");
            let r = desk.windows.find(who.0, who.1).expect("open").rect;
            for (px, py) in [(r.x, r.y), (r.x + r.width - 1, r.y + r.height - 1)] {
                assert_eq!(desk.hit(px, py), Some(who), "every corner of it is now its own");
            }
        }
    }
}

const SHELL: (u32, u32) = (5, 0x5442_4152);
const ROGUE: (u32, u32) = (66, 0x524F_4755);

/* The shell's dock is a popup window opened before any app window, so its z
 * is the lowest. The shell draws it over every window, and the hit test gives
 * it the press even under a window opened later; a popup of any other process
 * gets no such rank, and an app window covers it as its z says. */
#[test]
fn only_the_shells_popup_ranks_over_windows() {
    let mut desk = Desk::new();
    desk.open(SHELL, (440, 720, 400, 64), Kind::Popup);
    desk.open(ROGUE, (0, 0, 200, 200), Kind::Popup);
    desk.open(EDITOR, (0, 0, 1280, 800), Kind::Normal);
    let at = |x, y| topmost_hit_at(&desk.windows, x, y, SHELL.0).map(|h| h.owner_pid);
    assert_eq!(at(640, 740), Some(SHELL.0), "the dock, under the editor's z");
    assert_eq!(at(100, 100), Some(EDITOR.0), "another process's popup is an ordinary window");
    assert_eq!(at(640, 300), Some(EDITOR.0));
    // With no shell running, nothing is ranked over windows.
    assert_eq!(topmost_hit_at(&desk.windows, 640, 740, 0).map(|h| h.owner_pid), Some(EDITOR.0));
}
