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

//! The part that took a press keeps its events, in window coordinates.

use super::super::super::desk::event;
use super::super::super::press_part::{PressGrab, Route};
use super::rig::tabs;
use crate::input::InputKind;
use crate::press::Press;

#[test]
fn the_part_that_took_the_press_keeps_every_event_until_the_release() {
    let a = tabs().expect("a tab strip");
    let (in_x, in_y) = (a.x as i32 + 5, a.y as i32 + 5);
    let mut g = PressGrab::new();
    // A press on the window, then motion and a release over the tabs.
    assert!(matches!(g.route(event(InputKind::ButtonDown, 100, 25), tabs()), Route::Window(_)));
    assert!(matches!(g.route(event(InputKind::PointerAbs, in_x, in_y), tabs()), Route::Window(_)));
    assert!(matches!(g.route(event(InputKind::ButtonUp, in_x, in_y), tabs()), Route::Window(_)));
    // Released: hovering over the tabs reaches them again.
    assert!(matches!(
        g.route(event(InputKind::PointerAbs, in_x, in_y), tabs()),
        Route::Accessory(_)
    ));
    // A press on the tabs, dragged off them, stays theirs, in their coordinates.
    assert!(matches!(
        g.route(event(InputKind::ButtonDown, in_x, in_y), tabs()),
        Route::Accessory(_)
    ));
    match g.route(event(InputKind::PointerAbs, 100, 200), tabs()) {
        Route::Accessory(e) => assert_eq!((e.x, e.y), (100 - a.x as i32, 200 - a.y as i32)),
        Route::Window(_) => panic!("a drag off the tabs left them"),
    }
    assert!(matches!(g.route(event(InputKind::ButtonUp, 100, 200), tabs()), Route::Accessory(_)));
    assert!(matches!(g.route(event(InputKind::PointerAbs, 100, 200), tabs()), Route::Window(_)));
}

#[test]
fn keys_and_windows_without_a_widget_always_go_to_the_window() {
    let mut g = PressGrab::new();
    let a = tabs().expect("a tab strip");
    let key = event(InputKind::KeyDown, a.x as i32 + 1, a.y as i32 + 1);
    assert!(matches!(g.route(key, tabs()), Route::Window(_)));
    assert!(matches!(g.route(event(InputKind::ButtonDown, 600, 25), None), Route::Window(_)));
}

#[test]
fn the_router_press_frame_turns_screen_motion_into_the_same_window_delta() {
    let p = Press::arm(7, 260, 125, 160, 25);
    assert_eq!((p.origin_x, p.origin_y), (100, 100), "the window's origin at the press");
    assert_eq!(p.local(260, 125), (160, 25), "the press point itself");
    assert_eq!(p.local(300, 175), (200, 75), "moves by exactly the screen delta");
    assert_eq!(p.local(0, 0), (-100, -100), "left of and above the window is negative");
}
