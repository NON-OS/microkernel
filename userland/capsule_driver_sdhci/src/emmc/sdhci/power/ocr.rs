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

//! OCR voltage bits (JESD84 and the SD Physical Layer, as recalled,
//! unverified): bit 7 is 1.70-1.95 V, bits 15 to 23 are 2.7 to 3.6 V in
//! 100 mV steps, and bits 6:0 are reserved.

use super::super::caps::Caps;
use super::Vdd;

/// 1.70-1.95 V.
pub const OCR_165_195: u32 = 1 << 7;
/// 2.9-3.1 V.
pub const OCR_29_31: u32 = (1 << 17) | (1 << 18);
/// 3.2-3.4 V.
pub const OCR_32_34: u32 = (1 << 20) | (1 << 21);
/// Reserved; never part of a shared window.
pub const OCR_LOW_RESERVED: u32 = 0x7f;
/// What an eMMC device advertises: 1.70-1.95 V and 2.7-3.6 V.
pub const EMMC_DUAL_OCR: u32 = 0x00ff_8080;

/// The OCR bits naming each voltage the host's capabilities offer: the
/// supply it can give, as a window a card can be checked against.
pub const fn host_ocr(c: &Caps) -> u32 {
    given(c.v330(), OCR_32_34) | given(c.v300(), OCR_29_31) | given(c.v180(), OCR_165_195)
}

const fn given(offered: bool, bits: u32) -> u32 {
    if offered {
        bits
    } else {
        0
    }
}

/// The supply an OCR bit falls in, among the three a host can give.
pub const fn vdd_for_bit(bit: u32) -> Option<Vdd> {
    match bit {
        7 | 8 => Some(Vdd::V18),
        17 | 18 => Some(Vdd::V30),
        20..=23 => Some(Vdd::V33),
        _ => None,
    }
}
