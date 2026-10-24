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

//! Established timings III, the 0xF7 descriptor: a bitmap over 44 DMT
//! modes in bytes 6-11 (E-EDID 1.4 section 3.10.3.9, Table 3.30).

use crate::source::Source;
use crate::timing::Timing;

/// Width, height, rate in Hz and reduced blanking, from byte 6 bit 7
/// down to byte 11 bit 4; byte 11 bits 3-0 are reserved.
#[rustfmt::skip]
const TABLE: [(u32, u32, u32, bool); 44] = [
    (640, 350, 85, false), (640, 400, 85, false), (720, 400, 85, false),
    (640, 480, 85, false), (848, 480, 60, false), (800, 600, 85, false),
    (1024, 768, 85, false), (1152, 864, 75, false), (1280, 768, 60, true),
    (1280, 768, 60, false), (1280, 768, 75, false), (1280, 768, 85, false),
    (1280, 960, 60, false), (1280, 960, 85, false), (1280, 1024, 60, false),
    (1280, 1024, 85, false), (1360, 768, 60, false), (1440, 900, 60, true),
    (1440, 900, 60, false), (1440, 900, 75, false), (1440, 900, 85, false),
    (1400, 1050, 60, true), (1400, 1050, 60, false), (1400, 1050, 75, false),
    (1400, 1050, 85, false), (1680, 1050, 60, true), (1680, 1050, 60, false),
    (1680, 1050, 75, false), (1680, 1050, 85, false), (1600, 1200, 60, false),
    (1600, 1200, 65, false), (1600, 1200, 70, false), (1600, 1200, 75, false),
    (1600, 1200, 85, false), (1792, 1344, 60, false), (1792, 1344, 75, false),
    (1856, 1392, 60, false), (1856, 1392, 75, false), (1920, 1200, 60, true),
    (1920, 1200, 60, false), (1920, 1200, 75, false), (1920, 1200, 85, false),
    (1920, 1440, 60, false), (1920, 1440, 75, false),
];

/// Every mode whose bit is set in the descriptor `d` (18 bytes).
pub fn each(d: &[u8], f: &mut dyn FnMut(Timing)) {
    let Some(bits) = d.get(6..12) else { return };
    for (i, &(w, h, hz, rb)) in TABLE.iter().enumerate() {
        if bits[i / 8] & (0x80 >> (i % 8)) != 0 {
            let mut t = Timing::nominal(w, h, hz, Source::Established);
            t.reduced_blanking = rb;
            f(t);
        }
    }
}
