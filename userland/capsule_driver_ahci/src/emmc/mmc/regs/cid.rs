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

//! Card Identification, decoded from the R2 register CMD2 returns.

use super::r2::bits;

/// Card Identification (JEDEC eMMC 5.1, 7.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cid {
    pub mid: u8,
    pub cbx: u8,
    pub oid: u8,
    /// Product name, six ASCII bytes.
    pub pnm: [u8; 6],
    pub prv: u8,
    pub psn: u32,
    pub mdt: u8,
}

impl Cid {
    pub const fn parse(reg: u128) -> Self {
        let mut pnm = [0u8; 6];
        let mut i = 0;
        while i < 6 {
            // PNM is bits 103:56, first character highest.
            let hi = 103 - 8 * i as u32;
            pnm[i] = bits(reg, hi, hi - 7) as u8;
            i += 1;
        }
        Self {
            mid: bits(reg, 127, 120) as u8,
            cbx: bits(reg, 113, 112) as u8,
            oid: bits(reg, 111, 104) as u8,
            pnm,
            prv: bits(reg, 55, 48) as u8,
            psn: bits(reg, 47, 16),
            mdt: bits(reg, 15, 8) as u8,
        }
    }
}
