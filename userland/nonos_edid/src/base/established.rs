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

//! Established timings I and II, bytes 35-37 of the base block (E-EDID
//! 1.4 section 3.8, Table 3.18). Byte 37 bits 6-0 are the maker's own and
//! name nothing a driver can know.

use crate::source::Source;
use crate::timing::Timing;

/// Width, height, rate in Hz and interlace, from byte 35 bit 7 down to
/// byte 37 bit 7.
const TABLE: [(u32, u32, u32, bool); 17] = [
    (720, 400, 70, false),
    (720, 400, 88, false),
    (640, 480, 60, false),
    (640, 480, 67, false),
    (640, 480, 72, false),
    (640, 480, 75, false),
    (800, 600, 56, false),
    (800, 600, 60, false),
    (800, 600, 72, false),
    (800, 600, 75, false),
    (832, 624, 75, false),
    (1024, 768, 87, true),
    (1024, 768, 60, false),
    (1024, 768, 70, false),
    (1024, 768, 75, false),
    (1280, 1024, 75, false),
    (1152, 870, 75, false),
];

/// Every established timing whose bit is set in bytes 35-37.
pub fn each(base: &[u8], f: &mut dyn FnMut(Timing)) {
    let bits = u32::from_be_bytes([0, base[35], base[36], base[37]]);
    for (i, &(w, h, hz, interlaced)) in TABLE.iter().enumerate() {
        if bits & (1 << (23 - i)) != 0 {
            let mut t = Timing::nominal(w, h, hz, Source::Established);
            t.interlaced = interlaced;
            f(t);
        }
    }
}
