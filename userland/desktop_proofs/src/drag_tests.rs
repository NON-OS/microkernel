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

//! The frame's own pieces: the drag state machine, the press routing inside a
//! window, and the router's press grab, each driven directly.

use nonos_toolkit::decorations::{accessory_rect, Rect, TITLEBAR_H};

use super::desk::event;
use super::drag::{self, DragState, PointerAction};
use super::press_part::{PressGrab, Route};
use crate::input::InputKind;
use crate::press::Press;

const W: u32 = 700;
const H: u32 = 400;

fn press(state: &mut DragState, x: i32, y: i32) -> PointerAction {
    drag::handle(state, W, H, 100, 100, false, &event(InputKind::ButtonDown, x, y))
}

fn moved(action: PointerAction) -> Option<(u32, u32)> {
    match action {
        PointerAction::MoveTo(x, y) => Some((x, y)),
        _ => None,
    }
}

#[test]
fn a_press_on_the_title_bar_starts_a_drag_with_nothing_before_it() {
    let mut s = DragState::new();
    press(&mut s, 300, 25);
    assert!(s.active, "the first event the window sees is the press, and it drags");
    let a = drag::handle(&mut s, W, H, 100, 100, false, &event(InputKind::PointerAbs, 340, 75));
    assert_eq!(moved(a), Some((140, 150)), "the window moves by the pointer's delta");
}

#[test]
fn a_drag_never_puts_the_title_bar_under_the_menubar() {
    let mut s = DragState::new();
    press(&mut s, 300, 25);
    let a = drag::handle(&mut s, W, H, 100, 100, false, &event(InputKind::PointerAbs, 300, -400));
    let (_, y) = moved(a).expect("a move");
    assert_eq!(y, super::chrome::MENUBAR_H);
}

/// The titlebar's lowest pixels at its right end used to start a resize: the
/// check meant "below the titlebar" but compared the window-local y with the
/// menubar's height, 46, while the titlebar ends at 10 + 40 = 50.
#[test]
fn the_right_end_of_the_title_bar_moves_and_the_border_below_it_resizes() {
    let bottom = 10 + TITLEBAR_H;
    for y in [12, bottom as i32 - 4, bottom as i32 - 1] {
        let mut s = DragState::new();
        press(&mut s, W as i32 - 15, y);
        assert!(s.active, "a press at y {y} on the titlebar's right end moves the window");
    }
    let mut s = DragState::new();
    press(&mut s, W as i32 - 15, bottom as i32 + 1);
    assert!(!s.active, "just below the titlebar the right border resizes");
    let up = drag::handle(&mut s, W, H, 100, 100, false, &event(InputKind::PointerAbs, 800, 300));
    assert!(moved(up).is_none());
    let done = drag::handle(&mut s, W, H, 100, 100, false, &event(InputKind::ButtonUp, 800, 300));
    // The border moves as far as the pointer did, 115 to the right of the
    // press. It used to jump to the pointer plus the shadow margin, so this
    // press five pixels inside the border moved it five pixels before the
    // pointer had moved at all.
    assert!(matches!(done, PointerAction::ResizeTo(815, 400)), "resized by the pointer's travel");
}

#[test]
fn a_release_ends_the_drag() {
    let mut s = DragState::new();
    press(&mut s, 300, 25);
    drag::handle(&mut s, W, H, 100, 100, false, &event(InputKind::ButtonUp, 320, 40));
    assert!(!s.active);
    let a = drag::handle(&mut s, W, H, 100, 100, false, &event(InputKind::PointerAbs, 500, 200));
    assert!(moved(a).is_none(), "motion after the release is hover, not a drag");
}

#[test]
fn a_press_in_the_content_does_not_drag() {
    let mut s = DragState::new();
    press(&mut s, 300, 200);
    assert!(!s.active);
}

fn tabs() -> Option<Rect> {
    accessory_rect(W, H, false, 278)
}

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
