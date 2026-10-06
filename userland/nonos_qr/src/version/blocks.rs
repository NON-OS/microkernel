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

/* Error-correction codewords per block, by level L, M, Q, H. */
#[rustfmt::skip]
const EC_PER_BLOCK: [[u8; 40]; 4] = [
    [7,10,15,20,26,18,20,24,30,18,20,24,26,30,22,24,28,30,28,28,28,28,30,30,26,28,30,30,30,30,30,30,30,30,30,30,30,30,30,30],
    [10,16,26,18,24,16,18,22,22,26,30,22,22,24,24,28,28,26,26,26,26,28,28,28,28,28,28,28,28,28,28,28,28,28,28,28,28,28,28,28],
    [13,22,18,26,18,24,18,22,20,24,28,26,24,20,30,24,28,28,26,30,28,30,30,30,30,28,30,30,30,30,30,30,30,30,30,30,30,30,30,30],
    [17,28,22,16,22,28,26,26,24,28,24,28,22,24,24,30,28,28,26,28,30,24,30,30,30,30,30,30,30,30,30,30,30,30,30,30,30,30,30,30],
];

/* Error-correction blocks, by level L, M, Q, H. */
#[rustfmt::skip]
const BLOCKS: [[u8; 40]; 4] = [
    [1,1,1,1,1,2,2,2,2,4,4,4,4,4,6,6,6,6,7,8,8,9,9,10,12,12,12,13,14,15,16,17,18,19,19,20,21,22,24,25],
    [1,1,1,2,2,4,4,4,5,5,5,8,9,9,10,10,11,13,14,16,17,17,18,20,21,23,25,26,28,29,31,33,35,37,38,40,43,45,47,49],
    [1,1,2,2,4,4,6,6,8,8,8,10,12,16,12,17,16,18,21,20,23,23,25,27,29,34,34,35,38,40,43,45,48,51,53,56,59,62,65,68],
    [1,1,2,4,4,4,5,6,8,8,11,11,16,16,18,16,19,21,25,25,25,34,30,32,35,37,40,42,45,48,51,54,57,60,63,66,70,74,77,81],
];

/* The codewords a version's data and error-correction modules hold: every
 * module but the finders, separators, timing, alignment, format and
 * version information, eight to a codeword. */
pub(crate) fn total_codewords(version: u8) -> u16 {
    let v = version as u32;
    let mut modules = (16 * v + 128) * v + 64;
    if v >= 2 {
        let align = v / 7 + 2;
        modules -= (25 * align - 10) * align - 55;
        if v >= 7 {
            modules -= 36;
        }
    }
    (modules / 8) as u16
}

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
mod tests {
    use super::*;

    /* The hand-written table the encoder used for versions 1 to 10. */
    #[rustfmt::skip]
    const V1_10: [[(u16, u16, u16, u16, u16); 4]; 10] = [
        [(7,1,19,0,0),(10,1,16,0,0),(13,1,13,0,0),(17,1,9,0,0)],
        [(10,1,34,0,0),(16,1,28,0,0),(22,1,22,0,0),(28,1,16,0,0)],
        [(15,1,55,0,0),(26,1,44,0,0),(18,2,17,0,0),(22,2,13,0,0)],
        [(20,1,80,0,0),(18,2,32,0,0),(26,2,24,0,0),(16,4,9,0,0)],
        [(26,1,108,0,0),(24,2,43,0,0),(18,2,15,2,16),(22,2,11,2,12)],
        [(18,2,68,0,0),(16,4,27,0,0),(24,4,19,0,0),(28,4,15,0,0)],
        [(20,2,78,0,0),(18,4,31,0,0),(18,2,14,4,15),(26,4,13,1,14)],
        [(24,2,97,0,0),(22,2,38,2,39),(22,4,18,2,19),(26,4,14,2,15)],
        [(30,2,116,0,0),(22,3,36,2,37),(20,4,16,4,17),(24,4,12,4,13)],
        [(18,2,68,2,69),(26,4,43,1,44),(24,6,19,2,20),(28,6,15,2,16)],
    ];

    const LEVELS: [Ecc; 4] = [Ecc::Low, Ecc::Medium, Ecc::Quartile, Ecc::High];

    #[test]
    fn versions_1_to_10_are_the_table_they_replace() {
        for (v, row) in V1_10.iter().enumerate() {
            for (l, want) in row.iter().enumerate() {
                let b = blocks(v as u8 + 1, LEVELS[l]);
                let got = (b.ec_per_block, b.g1_blocks, b.g1_data, b.g2_blocks, b.g2_data);
                assert_eq!(got, *want, "version {} level {}", v + 1, l);
            }
        }
    }

    #[test]
    fn every_version_fills_exactly_its_modules() {
        for v in 1..=MAX_VERSION {
            for ecc in LEVELS {
                let b = blocks(v, ecc);
                let all = b.total_data_codewords() + b.total_blocks() * b.ec_per_block as usize;
                assert_eq!(all, total_codewords(v) as usize, "version {v}");
            }
        }
    }

    #[test]
    fn version_40_holds_iso_capacities() {
        // ISO/IEC 18004 Table 7: data codewords of version 40 at L, M, Q, H.
        let want = [2956, 2334, 1666, 1276];
        for (ecc, want) in LEVELS.iter().zip(want) {
            assert_eq!(blocks(40, *ecc).total_data_codewords(), want);
        }
        assert_eq!(total_codewords(40), 3706);
    }
}
