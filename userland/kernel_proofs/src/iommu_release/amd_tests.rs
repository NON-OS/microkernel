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

//! The same walk over AMD-Vi tables, whose directories name the level below.

use std::collections::HashMap;

use super::release_walk::release_walk;
use crate::amd_vi::pte::{directory, leaf, LEVELS};
use crate::amd_vi::pte_child::child_table;

#[test]
fn an_amd_domain_gives_back_its_directories_and_never_a_granted_frame() {
    let mut t: HashMap<u64, HashMap<usize, u64>> = HashMap::new();
    t.insert(0x1000, HashMap::from([(0, directory(0x2000, 4))]));
    t.insert(0x2000, HashMap::from([(0, directory(0x3000, 3))]));
    t.insert(0x3000, HashMap::from([(0, directory(0x4000, 2))]));
    t.insert(0x4000, HashMap::from([(3, leaf(0xA000, true, true))]));
    let read = |table: u64, i: usize| -> Result<u64, ()> {
        Ok(t.get(&table).ok_or(())?.get(&i).copied().unwrap_or(0))
    };
    let mut freed = Vec::new();
    let mut free = |table: u64| -> Result<(), ()> {
        freed.push(table);
        Ok(())
    };
    let n = release_walk(0x1000, LEVELS, &|e, l| child_table(e, l), &read, &mut free);
    assert_eq!(n, Ok(4));
    assert_eq!(freed, [0x4000, 0x3000, 0x2000, 0x1000]);
}

#[test]
fn an_amd_leaf_or_a_mislevelled_directory_leads_nowhere() {
    assert_eq!(child_table(directory(0x5000, 3), 3), Some(0x5000));
    assert_eq!(child_table(leaf(0x5000, true, true), 1), None);
    // A directory naming a level that is not the next one down is not
    // followed: freeing on a guess could hand back a granted frame.
    assert_eq!(child_table(directory(0x5000, 2), 4), None);
    assert_eq!(child_table(0, 3), None);
}
