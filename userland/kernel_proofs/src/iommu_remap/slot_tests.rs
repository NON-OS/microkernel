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

use super::regs::IRTE_ENTRIES;
use super::slots::{give, take, Slots};

#[test]
fn entries_are_handed_out_once_and_never_index_zero() {
    let mut slots: Slots = [0; 4];
    let taken: Vec<u16> = (0..300).filter_map(|_| take(&mut slots)).collect();
    assert_eq!(taken.len(), IRTE_ENTRIES as usize - 1);
    assert_eq!(taken[0], 1);
    assert!(!taken.contains(&0));
    assert!(give(&mut slots, 200));
    assert!(!give(&mut slots, 200), "double release refused");
    assert!(!give(&mut slots, 0) && !give(&mut slots, 256));
    assert_eq!(take(&mut slots), Some(200));
}
