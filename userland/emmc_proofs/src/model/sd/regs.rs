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

//! The model card's CID and CSD as the SD specification lays them out
//! (5.2, 5.3), and the capacity they give.

use super::SdCard;

impl SdCard {
    /// MID 127:120, OID 119:104, PNM 103:64, PRV 63:56, PSN 55:24, MDT
    /// 19:8 (2024, month 3), CRC 7:1, and the end bit.
    pub fn cid(&self) -> u128 {
        let c = &self.cfg;
        let mut v = (c.mid as u128) << 120 | (c.oid[0] as u128) << 112 | (c.oid[1] as u128) << 104;
        for (i, &b) in c.name.iter().enumerate() {
            v |= (b as u128) << (96 - 8 * i as u32);
        }
        v | (c.prv as u128) << 56 | (c.psn as u128) << 24 | 0x183u128 << 8 | 0x2bu128 << 1 | 1
    }

    /// CSD_STRUCTURE 127:126, TRAN_SPEED 103:96 (25 MHz), READ_BL_LEN
    /// 83:80, then C_SIZE where its version keeps it: 73:62 with
    /// C_SIZE_MULT 49:47 for 1.0, 69:48 for 2.0, 75:48 for 3.0.
    pub fn csd(&self) -> u128 {
        let c = &self.cfg;
        let base =
            (c.csd_structure as u128) << 126 | 0x32u128 << 96 | (c.read_bl_len as u128) << 80 | 1;
        match c.csd_structure {
            0 => base | (c.c_size as u128) << 62 | (c.c_size_mult as u128) << 47,
            _ => base | (c.c_size as u128) << 48,
        }
    }

    /// The card's size in 512-byte sectors.
    pub fn capacity(&self) -> u64 {
        let c = &self.cfg;
        match c.csd_structure {
            0 => ((c.c_size as u64 + 1) << (c.c_size_mult + 2) << c.read_bl_len) / 512,
            _ => (c.c_size as u64 + 1) * 1024,
        }
    }
}
