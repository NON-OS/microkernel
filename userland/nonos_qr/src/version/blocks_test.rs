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

use super::*;

/* A layout as (ec_per_block, g1_blocks, g1_data, g2_blocks, g2_data). */
type Layout = (u16, u16, u16, u16, u16);

/* The hand-written table the encoder used for versions 1 to 10. */
#[rustfmt::skip]
const V1_10: [[Layout; 4]; 10] = [
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
