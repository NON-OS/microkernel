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

//! The processor _OSC honour table, over every set of kernel features:
//! a bit is asked only when its feature is in the kernel, every bit a
//! feature honours is asked, no row is honoured by nothing, the bits are
//! distinct, and the kernel as it is today asks nothing.

use crate::caps::{word, word_for, Kernel, KERNEL, ROWS};

fn kernel(bits: u8) -> Kernel {
    let b = |n: u8| bits & (1 << n) != 0;
    Kernel {
        pstates: b(0),
        cstates_io: b(1),
        cstates_native: b(2),
        tstates: b(3),
        coordination: b(4),
        cppc: b(5),
    }
}

#[test]
fn a_bit_only_with_its_feature() {
    for bits in 0..64u8 {
        let k = kernel(bits);
        let w = word_for(&k);
        for r in &ROWS {
            assert_eq!(w & (1 << r.bit) != 0, (r.honoured)(&k), "{} with {k:?}", r.what);
        }
        let known: u32 = ROWS.iter().fold(0, |m, r| m | 1 << r.bit);
        assert_eq!(w & !known, 0, "no bit outside the table");
    }
}

#[test]
fn no_row_without_a_feature_and_nothing_today() {
    assert!(ROWS.iter().all(|r| !(r.honoured)(&Kernel::default())), "honoured by nothing");
    assert_eq!(KERNEL, Kernel::default(), "every feature false today");
    assert_eq!(word(), 0);
    let mut bits: Vec<u32> = ROWS.iter().map(|r| r.bit).collect();
    bits.dedup();
    assert_eq!(bits.len(), ROWS.len(), "each bit once");
}
