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
//! Specification 9.10, 5.1 to 5.3, as recalled, unverified): OCR and CID
//! here, the CSD in csd.rs. Pure, so the host proofs hold them.

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

pub use super::csd::{csd_agrees, sd_sectors};
