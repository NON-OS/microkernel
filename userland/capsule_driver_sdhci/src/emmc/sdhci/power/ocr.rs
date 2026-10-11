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

//! The OCR voltage bits and the host's and card's voltage windows.

use super::super::caps::Caps;
use super::Vdd;

/// OCR 1.65-1.95 V (bit 7; eMMC's 1.70-1.95 V window).
pub const OCR_165_195: u32 = 1 << 7;
/// OCR 2.9-3.0 and 3.0-3.1 V.
pub const OCR_29_31: u32 = (1 << 17) | (1 << 18);
/// OCR 3.2-3.3 and 3.3-3.4 V.
pub const OCR_32_34: u32 = (1 << 20) | (1 << 21);
/// OCR bits 6:0 are reserved for low voltage in MMC; Linux drops them.
pub const OCR_LOW_RESERVED: u32 = 0x7f;
/// The voltage window an eMMC device advertises: 1.70-1.95 V and 2.7-3.6 V.
pub const EMMC_DUAL_OCR: u32 = 0x00ff_8080;

/// The OCR bits the host can supply, as sdhci_setup_host builds ocr_avail.
pub const fn host_ocr(c: &Caps) -> u32 {
    let mut ocr = 0;
    if c.v330() {
        ocr |= OCR_32_34;
    }
    if c.v300() {
        ocr |= OCR_29_31;
    }
    if c.v180() {
        ocr |= OCR_165_195;
    }
    ocr
}

/// The voltage an OCR bit stands for, as sdhci_set_power maps ios.vdd.
pub const fn vdd_for_bit(bit: u32) -> Option<Vdd> {
    match bit {
        7 | 8 => Some(Vdd::V18),
        17 | 18 => Some(Vdd::V30),
        20..=23 => Some(Vdd::V33),
        _ => None,
    }
}
