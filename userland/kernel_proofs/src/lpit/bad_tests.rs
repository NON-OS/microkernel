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

//! Structures LPIT's walk skips or stops at.

use super::build::{gas, lpit, native};
use super::lpit_entry::{states, MAX_STATES};

#[test]
fn other_types_skipped_and_bad_structures_end_the_walk() {
    let other = [1u32.to_le_bytes(), 12u32.to_le_bytes(), [9; 4]].concat();
    let good = native(7, 0, 1, 2, &gas(1, 32, 0x1800), 0);
    assert_eq!(states(&lpit(&[other.clone(), good.clone()])).len(), 1);
    let short = [0u32.to_le_bytes(), 40u32.to_le_bytes()].concat();
    assert_eq!(
        states(&lpit(&[short, vec![0; 32], good.clone()])).len(),
        1,
        "short native: skipped"
    );
    let past = [0u32.to_le_bytes(), 200u32.to_le_bytes()].concat();
    assert_eq!(states(&lpit(&[good.clone(), past])).len(), 1, "past the table: walk ends");
    let zero = [0u32.to_le_bytes(), 0u32.to_le_bytes()].concat();
    assert_eq!(states(&lpit(&[zero, good.clone()])).len(), 0, "length 0 ends the walk");
    assert!(states(&lpit(&[])).is_empty() && states(&[0; 10]).is_empty());
    let many: Vec<_> = (0..40).map(|i| native(i, 0, 1, 2, &gas(1, 32, 0x1800), 0)).collect();
    assert_eq!(states(&lpit(&many)).len(), MAX_STATES);
}
