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
//! The photograph's credit, which CC BY 2.0 asks for wherever it is shown
//! (assets/etna/CREDITS.txt): at the band's foot, where the fade is ink.

use nonos_app_skeleton::PaintBuffer;

use super::super::backdrop::CREDIT;
use super::super::roles::Role;
use super::super::text::{draw, line, width};

/// Room left between the credit and the band's edges.
const PAD: i32 = 6;

/// Right-aligned on the last line of a band `column` wide and `h` tall at
/// `x, top`; left out when the band is too short to hold it clear.
pub(super) fn credit(fb: &mut PaintBuffer, x: u32, top: i64, column: u32, h: u32) {
    let tall = line(Role::Status);
    if i64::from(h) < i64::from(tall) * 3 {
        return;
    }
    let Ok(y) = i32::try_from(top + i64::from(h) - i64::from(tall + PAD)) else {
        return;
    };
    let right = x as i32 + column as i32 - PAD;
    let left = (right - width(Role::Status, CREDIT)).max(x as i32 + PAD);
    if y >= 0 {
        draw(fb, left, y, Role::Status, CREDIT);
    }
}
