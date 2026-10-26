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

//! The title bar drags, and nothing else does.

use super::super::super::desk::event;
use super::super::super::drag::{self, DragState};
use super::rig::{moved, press, H, W};
use crate::input::InputKind;

#[test]
fn a_press_on_the_title_bar_starts_a_drag_with_nothing_before_it() {
    let mut s = DragState::new();
    press(&mut s, 300, 25);
    assert!(s.active, "the first event the window sees is the press, and it drags");
    let a = drag::handle(&mut s, (W, H), (100, 100), false, &event(InputKind::PointerAbs, 340, 75));
    assert_eq!(moved(a), Some((140, 150)), "the window moves by the pointer's delta");
}

#[test]
fn a_drag_never_puts_the_title_bar_under_the_menubar() {
    let mut s = DragState::new();
    press(&mut s, 300, 25);
    let a =
        drag::handle(&mut s, (W, H), (100, 100), false, &event(InputKind::PointerAbs, 300, -400));
    let (_, y) = moved(a).expect("a move");
    assert_eq!(y, super::super::super::chrome::MENUBAR_H);
}

#[test]
fn a_release_ends_the_drag() {
    let mut s = DragState::new();
    press(&mut s, 300, 25);
    drag::handle(&mut s, (W, H), (100, 100), false, &event(InputKind::ButtonUp, 320, 40));
    assert!(!s.active);
    let a =
        drag::handle(&mut s, (W, H), (100, 100), false, &event(InputKind::PointerAbs, 500, 200));
    assert!(moved(a).is_none(), "motion after the release is hover, not a drag");
}

#[test]
fn a_press_in_the_content_does_not_drag() {
    let mut s = DragState::new();
    press(&mut s, 300, 200);
    assert!(!s.active);
}
