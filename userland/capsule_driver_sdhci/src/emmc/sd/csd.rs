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

//! The SD CSD as the driver reads it (SD Physical Layer Simplified
//! Specification 9.10, 5.3, as recalled, unverified): the card's size, and
//! whether its structure agrees with the capacity class its OCR claims.

use super::super::mmc::regs::bits;

/// The user area in 512-byte sectors, from the CSD. Version 1.0 (SDSC):
/// (C_SIZE 73:62 + 1) x 2^(C_SIZE_MULT 49:47 + 2) blocks of 2^READ_BL_LEN
/// 83:80 bytes. Version 2.0 (SDHC, SDXC): (C_SIZE 69:48 + 1) x 512 KiB.
/// None for version 3.0 (SDUC, past 2 TB: its addresses do not fit the 32
/// bits the read and write commands carry) and for a block length the
/// specification does not allow.
pub const fn sd_sectors(reg: u128) -> Option<u64> {
    match bits(reg, 127, 126) {
        0 => {
            let bl = bits(reg, 83, 80);
            if bl < 9 || bl > 11 {
                return None;
            }
            let blocks = (bits(reg, 73, 62) as u64 + 1) << (bits(reg, 49, 47) + 2);
            Some((blocks << bl) / 512)
        }
        1 => Some((bits(reg, 69, 48) as u64 + 1) * 1024),
        _ => None,
    }
}

/// CSD version 2.0 is the high and extended capacity cards' (5.3.3), and
/// only they set CCS; version 1.0 is a standard capacity card's, addressed
/// in bytes. A card whose two disagree would be addressed one way and
/// sized the other: past 4 GiB its byte addresses would wrap.
pub const fn csd_agrees(reg: u128, sector_mode: bool) -> bool {
    match bits(reg, 127, 126) {
        0 => !sector_mode,
        1 => sector_mode,
        _ => false,
    }
}
