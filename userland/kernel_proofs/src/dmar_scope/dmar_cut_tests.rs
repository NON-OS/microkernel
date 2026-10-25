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

use super::dmar_walk::{walk_structures, Walked, FIRST_STRUCTURE};

fn structure(kind: u16, length: u16) -> Vec<u8> {
    let mut v = kind.to_le_bytes().to_vec();
    v.extend_from_slice(&length.to_le_bytes());
    v.resize(length.max(4) as usize, 0);
    v
}

#[test]
fn a_cut_after_the_rmrrs_hides_none_of_them_and_one_before_may() {
    let bad = [0u8, 0, 2, 0];
    let mut t = vec![0u8; FIRST_STRUCTURE];
    t.extend(structure(0, 16));
    t.extend(structure(1, 24));
    let before = [t.as_slice(), &bad].concat();
    let cut = |past_rmrr| Walked { cut_short: true, past_rmrr };
    assert_eq!(walk_structures(&before, |_, _| {}), cut(false), "an RMRR may follow");
    t.extend(structure(2, 8));
    let after = [t.as_slice(), &bad].concat();
    assert_eq!(walk_structures(&after, |_, _| {}), cut(true), "an ATSR came first");
    assert_eq!(walk_structures(&t, |_, _| {}), Walked { cut_short: false, past_rmrr: true });
    // A kind no layout names says nothing about where the RMRRs are.
    let odd = [vec![0u8; FIRST_STRUCTURE], structure(9, 8), bad.to_vec()].concat();
    assert_eq!(walk_structures(&odd, |_, _| {}), cut(false));
    // Zeros out to the table's length are padding, not a structure cut short.
    let padded = [t.as_slice(), &[0u8; 12]].concat();
    assert_eq!(walk_structures(&padded, |_, _| {}), Walked { cut_short: false, past_rmrr: true });
}
