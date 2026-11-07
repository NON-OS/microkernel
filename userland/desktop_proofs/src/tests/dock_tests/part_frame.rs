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

//! A part frame composes the toasts' area with its change when toasts not
//! yet drawn go on it: opening an app raised the "opening a new window"
//! toast with the dock's pulse and the menu bar's new name, and the frame
//! was shown whole, all 1920x1080 pixels composed under every window.

use crate::render::layout::{dock_area_rect, menubar_rect, Rect};
use crate::taskbar::{part_frame, Area};

const SCREEN: (u32, u32) = (1920, 1080);
const TOASTS: Area = (1480, 890, 420, 96);
const REPAINT: &str = include_str!("../../../../capsule_desktop_shell/src/server/repaint.rs");
const TASKBAR: &str =
    include_str!("../../../../capsule_desktop_shell/src/server/refresh_taskbar.rs");

fn area(r: Rect) -> Area {
    (r.x, r.y, r.width, r.height)
}

fn strips() -> (Area, Area) {
    let (w, h) = SCREEN;
    (area(menubar_rect(w)), area(dock_area_rect(w, h)))
}

fn body<'a>(src: &'a str, head: &str) -> &'a str {
    let at = src.find(head).expect("the function");
    let end = src[at..].find("\n}\n").expect("its end");
    &src[at..at + end]
}

#[test]
fn new_toasts_are_composed_with_the_change_not_the_screen() {
    let (bar, dock) = strips();
    let parts = part_frame(&[bar, dock], Some(TOASTS), false, false).expect("a part frame");
    assert_eq!(parts.as_slice(), [bar, dock, TOASTS]);
    assert_eq!(parts.first(), bar, "presented with the change named");
    let px: u64 = parts.as_slice().iter().map(|&(_, _, w, h)| u64::from(w) * u64::from(h)).sum();
    let screen = u64::from(SCREEN.0) * u64::from(SCREEN.1);
    assert!(px * 4 < screen, "{px} px of {screen}");
}

#[test]
fn with_no_new_toast_the_change_alone() {
    let (bar, dock) = strips();
    let one = part_frame(&[dock], None, false, false).expect("a part frame");
    assert_eq!(one.as_slice(), [dock]);
    let two = part_frame(&[bar, dock], None, false, false).expect("a part frame");
    assert_eq!(two.as_slice(), [bar, dock]);
}

#[test]
fn a_dock_owed_the_lock_screen_or_nothing_named_is_shown_whole() {
    let (bar, dock) = strips();
    assert_eq!(part_frame(&[bar], Some(TOASTS), true, false), None, "the dock shown or hidden");
    assert_eq!(part_frame(&[bar], None, false, true), None, "the lock screen");
    assert_eq!(part_frame(&[], Some(TOASTS), false, false), None, "nothing named");
    assert_eq!(part_frame(&[bar, dock, bar], Some(TOASTS), false, false), None, "past the room");
    assert!(part_frame(&[bar, dock, bar], None, false, false).is_some(), "three fit");
}

#[test]
fn every_part_frame_takes_the_toasts_and_commits_only_once_shown() {
    let owed = body(REPAINT, "pub(super) fn owed_parts(");
    assert!(owed.contains("ctx.toast_view.toasts_drawn != ctx.toasts.generation()"));
    assert!(owed.contains("toast_rect(ctx.width, ctx.height)"));
    assert!(owed.contains("ctx.lock.screen.covers()"));
    let show = body(REPAINT, "pub(super) fn show_parts(");
    assert!(show.contains("let painted = paint_chrome_in(ctx, Changed::Part(x, y, w, h));"));
    assert!(show.contains("for &(x, y, w, h) in painted.owed().of(parts.as_slice()) {"));
    let part = body(REPAINT, "pub fn repaint_part(");
    assert!(part.contains("owed_parts(ctx, &[area(r)], dock_owed(ctx))"));
    let focus = body(REPAINT, "pub fn repaint_focus(");
    assert!(focus.contains("owed_parts(ctx, &strips, false)"), "the strips hold the dock");
    let dock = body(TASKBAR, "pub fn refresh_taskbar(");
    assert!(dock.contains("owed_parts(ctx, &[(r.x, r.y, r.width, r.height)], false)"));
    assert!(dock.contains("ctx.taskbar.drawn = ctx.taskbar.visible;"));
}
