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

//! A value as a QR code, drawn here so nothing leaves the machine to draw
//! it: medium correction, the four-module quiet zone, dark modules on the
//! paper colour with 12 of padding and the control corner, as the phones.

use nonos_app_skeleton::PaintBuffer;

use super::super::tokens::{CORNER, INK, TEXT};

const PAD: u32 = 12;
const QUIET: u32 = 4;

/// Draw a `side` square at `x, y`; false when the value does not encode.
pub fn qr(fb: &mut PaintBuffer, x: u32, y: u32, side: u32, value: &[u8]) -> bool {
    let Some(code) = nonos_qr::encode(value, nonos_qr::Ecc::Medium) else {
        return false;
    };
    fb.fill_round(x, y, side, side, CORNER, TEXT);
    let span = code.size as u32 + 2 * QUIET;
    let scale = ((side - 2 * PAD) / span).max(1);
    let drawn = span * scale;
    let ox = x + (side - drawn) / 2 + QUIET * scale;
    let oy = y + (side - drawn) / 2 + QUIET * scale;
    for my in 0..code.size {
        for mx in 0..code.size {
            if code.get(mx, my) {
                fb.fill_rect(ox + mx as u32 * scale, oy + my as u32 * scale, scale, scale, INK);
            }
        }
    }
    true
}
