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

//! Where the selected listing stands with this machine.

use nonos_app_skeleton::PaintBuffer;

use crate::store::state::State;
use crate::store::theme::{ACCENT, DANGER, MUTED, OK};
use crate::store::verdict::Verdict;

use super::gates;
use super::metrics::{BODY_PX, GATE_ROW_H, SMALL_PX};
use super::text;

/// Paints and returns the y to carry on from.
pub fn paint(fb: &mut PaintBuffer, state: &State, left: u32, mut top: i32) -> i32 {
    match state.ready {
        Some(r) => {
            let v = Verdict::of(&r);
            let hue = match v {
                Verdict::Ready => OK,
                Verdict::Installed => ACCENT,
                Verdict::Blocked => DANGER,
            };
            text::line(fb, left, top, v.sentence(), hue, BODY_PX);
            top += 30;
            gates::paint(fb, left, top, &r);
            top += 6 * GATE_ROW_H as i32 + 14;
        }
        None => {
            text::line(fb, left, top, b"checking", MUTED, SMALL_PX);
            top += 26;
        }
    }
    top
}
