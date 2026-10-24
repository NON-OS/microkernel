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

//! Two byte standard timings: eight at bytes 38-53 of the base block and
//! six more in each 0xFA descriptor (E-EDID 1.4 section 3.9, Table 3.19,
//! and section 3.10.3.8).

use crate::source::Source;
use crate::timing::Timing;

/// Decode one standard timing. Byte 0 is the width in 8 pixel steps less
/// 31; 01 01 marks an unused slot and 00 is reserved. Byte 1 bits 7-6
/// give the aspect ratio, which before EDID 1.3 began at 1:1, not 16:10;
/// bits 5-0 the rate less 60.
pub fn decode(b0: u8, b1: u8, at_least_1_3: bool) -> Option<Timing> {
    if b0 == 0 || (b0 == 1 && b1 == 1) {
        return None;
    }
    let w = (b0 as u32 + 31) * 8;
    let h = match b1 >> 6 {
        0 if at_least_1_3 => w * 10 / 16,
        0 => w,
        1 => w * 3 / 4,
        2 => w * 4 / 5,
        _ => w * 9 / 16,
    };
    Some(Timing::nominal(w, h, (b1 & 0x3F) as u32 + 60, Source::Standard))
}

/// Every used standard timing in `pairs`, two bytes apiece.
pub fn each(pairs: &[u8], at_least_1_3: bool, f: &mut dyn FnMut(Timing)) {
    for p in pairs.chunks_exact(2) {
        if let Some(t) = decode(p[0], p[1], at_least_1_3) {
            f(t);
        }
    }
}
