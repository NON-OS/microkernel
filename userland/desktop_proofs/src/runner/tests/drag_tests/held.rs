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

//! A resize keeps its events from the app. The edge bands lie over the
//! window's own pixels, and the press that started a resize used to go on to
//! the app too: in Settings it chose the sidebar row under the left edge.

use super::super::super::desk::event;
use super::super::super::drag::{self, DragState, PointerAction};
use super::rig::{press, H, W};
use crate::input::InputKind;

fn at(s: &mut DragState, kind: InputKind, x: i32, y: i32) -> PointerAction {
    drag::handle(s, (W, H), (100, 100), false, &event(kind, x, y))
}

#[test]
fn frame_held_resize_never_reaches_the_app() {
    let mut s = DragState::new();
    assert!(matches!(press(&mut s, 3, 200), PointerAction::Held), "the press on the edge");
    let step = at(&mut s, InputKind::PointerAbs, -40, 200);
    assert!(matches!(step, PointerAction::Held), "the motion of the resize");
    assert!(matches!(at(&mut s, InputKind::ButtonUp, -40, 200), PointerAction::Place(_)));
}

#[test]
fn frame_held_release_where_it_began_applies_nothing() {
    let mut s = DragState::new();
    press(&mut s, W as i32 - 3, 200);
    assert!(matches!(at(&mut s, InputKind::ButtonUp, W as i32 - 3, 200), PointerAction::Held));
}

#[test]
fn frame_held_is_only_the_frames() {
    let mut s = DragState::new();
    assert!(matches!(press(&mut s, 300, 200), PointerAction::None), "a press in the content");
    assert!(matches!(at(&mut s, InputKind::PointerAbs, 310, 210), PointerAction::None));
    assert!(matches!(at(&mut s, InputKind::ButtonUp, 310, 210), PointerAction::None));
}
