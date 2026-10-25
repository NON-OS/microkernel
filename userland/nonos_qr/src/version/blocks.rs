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

//! The error-correction block layout of every version, 1 to 40, derived as
//! ISO/IEC 18004 Table 9 is: from each version's error-correction codewords
//! per block and its number of blocks, the codewords its modules hold, and
//! the rule that the first blocks are one data codeword shorter.

use super::block_tables::{total_codewords, BLOCKS, EC_PER_BLOCK};
use super::ecc::Ecc;

pub(crate) struct Blocks {
    pub ec_per_block: u16,
    pub g1_blocks: u16,
    pub g1_data: u16,
    pub g2_blocks: u16,
    pub g2_data: u16,
}

impl Blocks {
    pub(crate) fn total_data_codewords(&self) -> usize {
        (self.g1_blocks * self.g1_data + self.g2_blocks * self.g2_data) as usize
    }
    pub(crate) fn total_blocks(&self) -> usize {
        (self.g1_blocks + self.g2_blocks) as usize
    }
}

pub(crate) const MAX_VERSION: u8 = 40;

pub(crate) fn blocks(version: u8, ecc: Ecc) -> Blocks {
    let at = (version - 1) as usize;
    let ec = EC_PER_BLOCK[ecc.index()][at] as u16;
    let n = BLOCKS[ecc.index()][at] as u16;
    let total = total_codewords(version);
    let long = total % n;
    let short_data = total / n - ec;
    Blocks {
        ec_per_block: ec,
        g1_blocks: n - long,
        g1_data: short_data,
        g2_blocks: long,
        g2_data: if long == 0 { 0 } else { short_data + 1 },
    }
}

#[cfg(test)]
#[path = "blocks_test.rs"]
mod tests;
