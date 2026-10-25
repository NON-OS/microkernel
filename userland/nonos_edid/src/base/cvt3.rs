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

//! Three byte CVT codes, four to a 0xF8 descriptor at bytes 6-17 (E-EDID
//! 1.4 section 3.10.3.11, Tables 3.33 and 3.34).

use crate::source::Source;
use crate::timing::Timing;

/// The rates byte 2 bits 4-0 offer, bit 4 first; bit 0 is 60 Hz with
/// reduced blanking.
const RATES: [(u32, bool); 5] = [(50, false), (60, false), (75, false), (85, false), (60, true)];

/// Every timing one code offers. Byte 0 and byte 1 bits 7-4 give the
/// lines as (n + 1) * 2; byte 1 bits 3-2 the aspect ratio, from which the
/// width is the lines scaled and cut to a multiple of 8 pixels. A code
/// of all zero bytes is an empty slot.
pub fn code(c: &[u8], f: &mut dyn FnMut(Timing)) {
    let [b0, b1, b2] = *c else { return };
    if b0 == 0 && b1 == 0 && b2 == 0 {
        return;
    }
    let lines = ((b0 as u32 | ((b1 as u32 >> 4) << 8)) + 1) * 2;
    let (num, den) = match (b1 >> 2) & 3 {
        0 => (4, 3),
        1 => (16, 9),
        2 => (16, 10),
        _ => (15, 9),
    };
    let width = lines * num / den / 8 * 8;
    // A handful of lines cuts to no width at all: no picture to offer.
    if width == 0 {
        return;
    }
    for (bit, &(hz, rb)) in RATES.iter().enumerate() {
        if b2 & (0x10 >> bit) != 0 {
            let mut t = Timing::nominal(width, lines, hz, Source::Cvt);
            t.reduced_blanking = rb;
            f(t);
        }
    }
}

/// Every code in a 0xF8 descriptor `d` (18 bytes).
pub fn each(d: &[u8], f: &mut dyn FnMut(Timing)) {
    for c in d.get(6..18).unwrap_or(&[]).chunks_exact(3) {
        code(c, f);
    }
}
