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

//! The SD registers as the driver reads them (SD Physical Layer Simplified
//! Specification 9.10, 5.1 to 5.3, as recalled, unverified): OCR, CID and CSD. Pure, so the host
//! proofs hold them.

use super::super::mmc::regs::bits;
use super::super::mmc::Cid;

/// OCR bit 30, Card Capacity Status: valid once power up is done. Set, the
/// card is addressed in 512-byte blocks; clear, in bytes.
pub const OCR_CCS: u32 = 1 << 30;

pub const fn ccs(ocr: u32) -> bool {
    ocr & OCR_CCS != 0
}

/// The SD CID in the driver's CID form: MID 127:120, the two-character OID
/// 119:104 (its first byte kept), the five-character PNM 103:64 padded
/// with a space, PRV 63:56, PSN 55:24, and the year byte of MDT 19:12.
pub const fn sd_cid(reg: u128) -> Cid {
    let mut pnm = [b' '; 6];
    let mut i = 0;
    while i < 5 {
        let hi = 103 - 8 * i as u32;
        pnm[i] = bits(reg, hi, hi - 7) as u8;
        i += 1;
    }
    Cid {
        mid: bits(reg, 127, 120) as u8,
        cbx: 0,
        oid: bits(reg, 119, 112) as u8,
        pnm,
        prv: bits(reg, 63, 56) as u8,
        psn: bits(reg, 55, 24),
        mdt: bits(reg, 19, 12) as u8,
    }
}

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
