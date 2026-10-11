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

use crate::spent::{SpentSet, SPENT_CAPACITY};

#[test]
fn the_spent_set_reports_when_it_wraps() {
    let mut spent = SpentSet::new();
    for i in 0..SPENT_CAPACITY {
        let mut tx = [0u8; 32];
        tx[0..8].copy_from_slice(&(i as u64).to_le_bytes());
        assert!(!spent.record(tx), "no eviction while filling");
    }
    assert_eq!(spent.len(), SPENT_CAPACITY);
    // One more evicts the oldest.
    assert!(spent.record([0xFF; 32]));
}
