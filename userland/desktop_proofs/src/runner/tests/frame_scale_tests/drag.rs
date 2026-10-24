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

//! The title bar drags and the border below it resizes, at every scale.

use super::super::super::desk::event;
use super::super::super::drag::{handle_at, DragState, PointerAction};
use super::rig::{px, H, SCALES, W};
use crate::input::InputKind;
use nonos_toolkit::decorations::{light_rect_at, titlebar_rect_at};

#[test]
fn the_title_bar_drags_and_the_border_below_it_resizes_at_every_scale() {
    for q in SCALES {
        let bar = titlebar_rect_at(W, H, false, q);
        let bottom = (bar.y + bar.h) as i32;
        // The right end of the bar's lowest row moves the window.
        let mut s = DragState::new();
        let at = event(InputKind::ButtonDown, W as i32 - px(15, q) as i32, bottom - 1);
        handle_at(&mut s, (W, H), (100, 100), false, &at, q);
        assert!(s.active, "the bar's last row moves the window at {q}");
        let to = event(InputKind::PointerAbs, W as i32 - px(15, q) as i32 + 40, bottom + 29);
        let a = handle_at(&mut s, (W, H), (100, 100), false, &to, q);
        assert!(matches!(a, PointerAction::MoveTo(140, 130)), "moves by the delta at {q}");

        // Just below it the right border resizes, by the pointer's travel.
        let mut s = DragState::new();
        let at = event(InputKind::ButtonDown, W as i32 - px(15, q) as i32, bottom + 1);
        handle_at(&mut s, (W, H), (100, 100), false, &at, q);
        assert!(!s.active, "below the bar the border resizes at {q}");
        let to = event(InputKind::PointerAbs, 1000, 300);
        handle_at(&mut s, (W, H), (100, 100), false, &to, q);
        let up = event(InputKind::ButtonUp, 1000, 300);
        let done = handle_at(&mut s, (W, H), (100, 100), false, &up, q);
        let want = W + 1000 - (W - px(15, q));
        assert!(matches!(done, PointerAction::ResizeTo(w, 600) if w == want), "resize at {q}");

        // A press on the close button neither drags nor resizes.
        let close = light_rect_at(0, W, H, false, q);
        let mut s = DragState::new();
        let at = event(InputKind::ButtonDown, (close.x + 1) as i32, (close.y + 1) as i32);
        handle_at(&mut s, (W, H), (100, 100), false, &at, q);
        assert!(!s.active, "the close button is not a drag at {q}");
    }
}
