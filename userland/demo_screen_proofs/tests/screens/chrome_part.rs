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

//! The shell's repaints that show only part of the frame (server/repaint.rs
//! repaint_part) change nothing outside that part: a row lit in an open
//! menu changes only the menu's panel (render/desktop_menu/frame.rs and
//! render/menubar_menu/frame.rs), and a step of an icon drag only where the
//! icon was and is (render/desktop_icons/ghost.rs). The shell's own painter
//! draws each frame here.

use demo_screen_proofs::desk::{paint_seen, Desk};
use demo_screen_proofs::render::layout::{dock_area_rect, Rect};
use demo_screen_proofs::render::toasts::place::toast_rect;
use demo_screen_proofs::render::{desktop_icons, desktop_menu, menubar_menu};
use demo_screen_proofs::server::dock_clock;
use demo_screen_proofs::state::{
    mark_taskbar_launch, track_window_closed, track_window_opened, Context, NotifyLevel,
};
use demo_screen_proofs::vfs_client::Entry;

const SIZES: [(u32, u32); 2] = [(1280, 800), (1920, 1080)];

fn changed_outside(w: u32, a: &Desk, b: &Desk, r: Rect) -> Vec<(u32, u32)> {
    let inside = |x: u32, y: u32| x >= r.x && x < r.x + r.width && y >= r.y && y < r.y + r.height;
    let px = |i: usize| (i as u32 % w, i as u32 / w);
    let differ =
        |p: &[u32], q: &[u32]| -> Vec<usize> { (0..p.len()).filter(|&i| p[i] != q[i]).collect() };
    let mut out: Vec<_> = differ(&a.chrome, &b.chrome).into_iter().map(px).collect();
    out.extend(differ(&a.desk, &b.desk).into_iter().map(px));
    out.retain(|&(x, y)| !inside(x, y));
    out
}

#[test]
fn a_row_lit_in_the_desktop_menu_changes_its_panel_only() {
    for (w, h) in SIZES {
        let open = |hover: Option<usize>| {
            paint_seen(
                w,
                h,
                |c| (c.desk_ui.desktop_menu, c.desk_ui.menu_hover) = (Some((w / 3, h / 3)), hover),
                desktop_menu::frame,
            )
        };
        let (rest, frame) = open(None);
        let frame = frame.expect("the menu is open");
        for row in 0..2 {
            let (lit, again) = open(Some(row));
            assert_eq!(again.map(|r| (r.x, r.y)), Some((frame.x, frame.y)));
            assert_ne!(lit.chrome, rest.chrome, "{w}x{h}: row {row} is drawn lit");
            let out = changed_outside(w, &rest, &lit, frame);
            assert!(out.is_empty(), "{w}x{h}: row {row} changed {:?} outside", out.first());
        }
    }
}

#[test]
fn a_row_lit_in_a_menubar_drop_down_changes_its_panel_only() {
    for (w, h) in SIZES {
        for title in 1..3 {
            let open = |hover: Option<usize>| {
                paint_seen(
                    w,
                    h,
                    |c| (c.menubar.open, c.menubar.hover) = (Some(title), hover),
                    menubar_menu::frame,
                )
            };
            let (rest, frame) = open(None);
            let frame = frame.expect("the drop-down is open");
            for row in 0..2 {
                let (lit, _) = open(Some(row));
                assert_ne!(lit.chrome, rest.chrome, "{w}x{h} title {title}: row {row} is lit");
                let out = changed_outside(w, &rest, &lit, frame);
                assert!(out.is_empty(), "{w}x{h} title {title}: row {row}: {:?}", out.first());
            }
        }
    }
}

fn dragging(c: &mut Context, at: (u32, u32)) {
    let item = |name: &str, is_dir| Entry { name: name.into(), is_dir, children: 0 };
    c.desktop_items = vec![item("notes", false), item("photos", true)];
    c.desk_ui.drag_from = Some(1);
    c.desk_ui.drag_moved = true;
    (c.desk_ui.drag_x, c.desk_ui.drag_y) = at;
}

/// An icon dragged a step changes the chrome only where it was drawn and
/// where it is drawn now (render/desktop_icons/ghost.rs).
#[test]
fn a_dragged_icon_changes_only_where_it_was_and_is() {
    for (w, h) in SIZES {
        let steps =
            [((w / 2, h / 2), (w / 2 + 9, h / 2 + 4)), ((w / 3, h / 4), (w / 3 + 200, h / 4))];
        for (from, to) in steps {
            let (a, was) = paint_seen(w, h, |c| dragging(c, from), desktop_icons::ghost_rect);
            let (b, now) = paint_seen(w, h, |c| dragging(c, to), desktop_icons::ghost_rect);
            let (was, now) = (was.expect("drawn"), now.expect("drawn"));
            assert_ne!(a.chrome, b.chrome, "{w}x{h}: the icon moved");
            let within = |(x, y): &(u32, u32), r: Rect| {
                *x >= r.x && *x < r.x + r.width && *y >= r.y && *y < r.y + r.height
            };
            let stray: Vec<_> =
                changed_outside(w, &a, &b, was).into_iter().filter(|p| !within(p, now)).collect();
            assert!(stray.is_empty(), "{w}x{h}: {from:?} to {to:?}: {:?}", stray.first());
        }
    }
}

/// What a refresh of the taskbar shows (server/refresh_taskbar.rs): the
/// dock shown or hidden, a window of an app that is not the active one
/// closed, a launch's pulse. None of them changes anything outside the
/// dock's area, panel and shadow (render/layout dock_area_rect). A change
/// of the active app also renames the menubar's app, so it changes the frame
/// outside that area and is shown whole instead (server/refresh_taskbar.rs
/// refresh_after), whether its window closed or a dock click forgot it.
#[test]
fn what_a_taskbar_refresh_shows_changes_only_the_dock_area() {
    let two = |c: &mut Context| {
        c.taskbar.visible = true;
        track_window_opened(&mut c.taskbar, 7, 70, 0);
        track_window_opened(&mut c.taskbar, 8, 80, 1);
    };
    for (w, h) in SIZES {
        let area = dock_area_rect(w, h);
        let (base, active) = paint_seen(w, h, two, |c| c.taskbar.active);
        let (hidden, _) = paint_seen(w, h, |c| (two(c), c.taskbar.visible = false).1, |_| ());
        let closed = |c: &mut Context| {
            two(c);
            let _ = track_window_closed(&mut c.taskbar, 70, 7);
        };
        let (other_closed, still) = paint_seen(w, h, closed, |c| c.taskbar.active);
        assert_eq!(still, active, "closing the other app's window keeps the active app");
        let pulse =
            |c: &mut Context| (two(c), mark_taskbar_launch(&mut c.taskbar, 2, dock_clock::now())).1;
        let (pulsed, _) = paint_seen(w, h, pulse, |_| ());
        for (what, other) in [("hidden", &hidden), ("closed", &other_closed), ("pulsed", &pulsed)] {
            assert_ne!(base.chrome, other.chrome, "{w}x{h}: dock {what} is drawn");
            let out = changed_outside(w, &base, other, area);
            assert!(out.is_empty(), "{w}x{h}: dock {what}: {:?} outside", out.first());
        }
        let gone = |c: &mut Context| {
            two(c);
            let _ = track_window_closed(&mut c.taskbar, 80, 8);
        };
        let (active_gone, moved) = paint_seen(w, h, gone, |c| c.taskbar.active);
        assert_ne!(moved, active, "{w}x{h}: forgetting the active app's window moves it");
        let out = changed_outside(w, &base, &active_gone, area);
        assert!(!out.is_empty(), "{w}x{h}: the menubar names another app, outside the dock");
    }
}

/// A toast raised or gone changes nothing outside the toasts' rectangle
/// (render/toasts/place.rs toast_rect), the part their sync shows
/// (render/toasts/layer.rs).
#[test]
fn a_toast_changes_only_the_toasts_rectangle() {
    let now = demo_screen_proofs::server::toast_clock::now;
    for (w, h) in SIZES {
        let r = toast_rect(w, h);
        let (none, _) = paint_seen(w, h, |_| (), |_| ());
        let one = |c: &mut Context| c.toasts.push(b"network connected", NotifyLevel::Info, now());
        let (shown, _) = paint_seen(w, h, one, |_| ());
        let two = |c: &mut Context| {
            one(c);
            c.toasts.push(b"a longer second line of words", NotifyLevel::Warn, now());
        };
        let (both, _) = paint_seen(w, h, two, |_| ());
        for (what, a, b) in [("one", &none, &shown), ("two", &shown, &both)] {
            assert_ne!(a.chrome, b.chrome, "{w}x{h}: {what} toast drawn");
            let out = changed_outside(w, a, b, r);
            assert!(out.is_empty(), "{w}x{h}: {what}: {:?} outside", out.first());
        }
    }
}

/// The right-click menu opening or going away, on the empty desktop or on
/// an icon, changes nothing outside its panel and shadow
/// (render/desktop_menu/frame.rs reach), the part server/input/menu.rs
/// shows for it.
#[test]
fn the_desktop_menu_opening_changes_only_its_panel_and_shadow() {
    for (w, h) in SIZES {
        let icons = |c: &mut Context| dragging(c, (0, 0));
        let still = |c: &mut Context| (icons(c), c.desk_ui.drag_from = None).1;
        let (closed, _) = paint_seen(w, h, still, |_| ());
        for (target, at) in [(None, (w / 3, h / 3)), (Some(1), (w / 2, h / 2))] {
            let open = |c: &mut Context| {
                still(c);
                c.desk_ui.menu_target = target;
                c.desk_ui.desktop_menu = Some(at);
            };
            let (shown, reach) = paint_seen(w, h, open, desktop_menu::reach);
            let reach = reach.expect("the menu is open");
            assert_ne!(closed.chrome, shown.chrome, "{w}x{h}: the menu is drawn");
            let out = changed_outside(w, &closed, &shown, reach);
            assert!(out.is_empty(), "{w}x{h} {target:?}: {:?} outside", out.first());
        }
    }
}
