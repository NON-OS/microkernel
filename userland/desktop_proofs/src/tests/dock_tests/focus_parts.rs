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

//! A change of the focused app composes the menu bar and the dock's area,
//! never the screen whole: a press on the title bar of a window behind
//! another composed all 1920x1080 pixels before the drag it started moved.

use crate::render::layout::{dock_area_rect, menubar_rect, Rect};
use crate::taskbar::{focus_parts, Area};

const SCREEN: (u32, u32) = (1920, 1080);

fn area(r: Rect) -> Area {
    (r.x, r.y, r.width, r.height)
}

fn strips() -> (Area, Area) {
    let (w, h) = SCREEN;
    (area(menubar_rect(w)), area(dock_area_rect(w, h)))
}

fn pixels(a: &[Area]) -> u64 {
    a.iter().map(|&(_, _, w, h)| u64::from(w) * u64::from(h)).sum()
}

#[test]
fn a_focus_change_composes_the_bar_and_the_dock_alone() {
    let (bar, dock) = strips();
    let parts = focus_parts(bar, dock, false, false).expect("no whole frame owed");
    assert_eq!(parts, [bar, dock], "the bar first, the frame presented with it named");
    let screen = u64::from(SCREEN.0) * u64::from(SCREEN.1);
    let composed = pixels(&parts);
    assert!(composed * 4 < screen, "{composed} px of {screen}: a quarter of the screen at most");
}

#[test]
fn the_two_strips_do_not_meet_so_no_box_joins_them() {
    let (bar, dock) = strips();
    assert_eq!(bar.1, 0, "the bar is at the top");
    assert!(bar.1 + bar.3 < dock.1, "the dock's area starts below the bar");
    assert_eq!(dock.1 + dock.3, SCREEN.1, "the dock's area reaches the bottom edge");
}

#[test]
fn a_frame_owed_whole_or_an_open_drop_down_is_shown_whole() {
    let (bar, dock) = strips();
    assert_eq!(
        focus_parts(bar, dock, true, false),
        None,
        "the dock shown or hidden, the lock screen"
    );
    assert_eq!(focus_parts(bar, dock, false, true), None, "the drop-down's rows follow the app");
}

#[test]
fn the_focused_notice_and_a_close_that_moved_the_app_repaint_only_the_strips() {
    const NOTIFY: &str = include_str!("../../../../capsule_desktop_shell/src/server/wm_notify.rs");
    assert!(NOTIFY.contains(
        "if focus_moved(&mut ctx.taskbar, app) {\n                super::repaint::repaint_focus(ctx);"
    ));
    const AFTER: &str =
        include_str!("../../../../capsule_desktop_shell/src/server/refresh_taskbar.rs");
    assert!(AFTER
        .contains("if ctx.taskbar.active != named {\n        super::repaint::repaint_focus(ctx);"));
    const REPAINT: &str = include_str!("../../../../capsule_desktop_shell/src/server/repaint.rs");
    let focus = REPAINT.split("pub fn repaint_focus").nth(1).expect("repaint_focus");
    let body = focus.split("\n}\n").next().expect("its body");
    assert!(!body.contains("ctx.width, ctx.height);\n    ctx.taskbar"), "no whole commit");
    assert!(body.contains("show_parts(ctx, parts);"), "the strips, then what else is owed");
    assert!(body.contains("return repaint(ctx);"), "owed whole, shown whole");
    let show = REPAINT.split("pub(super) fn show_parts").nth(1).expect("show_parts");
    let show = show.split("\n}\n").next().expect("its body");
    assert!(show.contains("Changed::Part(x, y, w, h)"), "presented with the bar named");
    assert!(show.contains("painted.owed().of(parts.as_slice())"), "committed only what is owed");
}
