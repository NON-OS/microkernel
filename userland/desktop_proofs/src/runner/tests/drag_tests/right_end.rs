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

//! The title bar's right end moves; the border below it resizes.

use super::super::super::desk::event;
use super::super::super::drag::{self, DragState, PointerAction};
use super::rig::{moved, press, H, W};
use crate::input::InputKind;
use nonos_toolkit::decorations::TITLEBAR_H;

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
