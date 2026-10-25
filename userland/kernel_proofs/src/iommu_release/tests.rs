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

use std::collections::HashMap;

use super::release_walk::{child_table, release_walk};
use super::sl_pte::{leaf, table, SL_LARGE};

/// Tables by frame address, each a sparse map of index to entry.
type Tables = HashMap<u64, HashMap<usize, u64>>;

fn run(tables: &Tables, root: u64, levels: u8) -> Result<(usize, Vec<u64>), &'static str> {
    let read = |t: u64, i: usize| -> Result<u64, &'static str> {
        let entries = tables.get(&t).ok_or("walked into a frame that is not a table")?;
        Ok(entries.get(&i).copied().unwrap_or(0))
    };
    let mut freed = Vec::new();
    let mut free = |t: u64| -> Result<(), &'static str> {
        freed.push(t);
        Ok(())
    };
    let n = release_walk(root, levels, &|e, l| child_table(e, l), &read, &mut free)?;
    Ok((n, freed))
}

#[test]
fn every_table_goes_back_children_first_and_no_granted_frame_does() {
    // A four-level domain with two grants under one level-1 table, one 2 MiB
    // large page, and a second level-2 branch.
    let mut t: Tables = HashMap::new();
    t.insert(0x1000, HashMap::from([(0, table(0x2000))]));
    t.insert(0x2000, HashMap::from([(0, table(0x3000)), (5, table(0x5000))]));
    t.insert(
        0x3000,
        HashMap::from([(0, table(0x4000)), (1, leaf(0x20_0000, true, true, true) | SL_LARGE)]),
    );
    t.insert(
        0x4000,
        HashMap::from([(0, leaf(0xA000, true, false, true)), (9, leaf(0xB000, false, true, true))]),
    );
    t.insert(0x5000, HashMap::new());
    let (n, freed) = run(&t, 0x1000, 4).expect("walked");
    assert_eq!(n, 5);
    assert_eq!(freed, [0x4000, 0x3000, 0x5000, 0x2000, 0x1000]);
    for granted in [0xA000, 0xB000, 0x20_0000] {
        assert!(!freed.contains(&granted), "{granted:#x} belongs to a grant");
    }
}

#[test]
fn an_empty_domain_gives_back_its_root_alone() {
    let t: Tables = HashMap::from([(0x1000, HashMap::new())]);
    assert_eq!(run(&t, 0x1000, 4).expect("walked"), (1, vec![0x1000]));
}

#[test]
fn leaves_large_pages_and_empty_entries_lead_nowhere() {
    assert_eq!(child_table(table(0x7000), 2), Some(0x7000));
    assert_eq!(child_table(table(0x7000), 1), None, "level 1 holds leaves only");
    assert_eq!(child_table(leaf(0x7000, true, true, true) | SL_LARGE, 3), None);
    assert_eq!(child_table(0, 4), None);
}
