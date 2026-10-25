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

//! A destroyed domain gives back its tables, children first, and never a
//! leaf's frame (K12).

use std::collections::HashMap;

use super::frames_of::table_frames;

const PRESENT: u64 = 1;
const LARGE: u64 = 1 << 7;

/// A three-level table: root 0x1000 names 0x2000 and 0x3000; 0x2000 names
/// 0x4000; 0x3000 holds a large page; 0x4000 holds leaves at 0x9000 on.
fn memory() -> HashMap<u64, Vec<u64>> {
    let table = |entries: &[u64]| {
        let mut t = vec![0u64; 512];
        t[..entries.len()].copy_from_slice(entries);
        t
    };
    HashMap::from([
        (0x1000, table(&[0x2000 | PRESENT, 0, 0x3000 | PRESENT])),
        (0x2000, table(&[0x4000 | PRESENT])),
        (0x3000, table(&[0x20_0000 | PRESENT | LARGE])),
        (0x4000, table(&[0x9000 | PRESENT, 0xA000 | PRESENT])),
    ])
}

fn child(entry: u64, level: u8) -> Option<u64> {
    let table = level > 1 && entry & PRESENT != 0 && entry & LARGE == 0;
    table.then_some(entry & !0xFFF)
}

#[test]
fn every_table_comes_back_children_first_and_no_leaf() {
    let mem = memory();
    let read = |p: u64| mem.get(&p).map(|t| t.as_slice());
    let frames = table_frames(0x1000, 3, &read, &child).unwrap();
    assert_eq!(frames, [0x4000, 0x2000, 0x3000, 0x1000]);
    assert!(!frames.contains(&0x9000) && !frames.contains(&0x20_0000), "data stays");
}

#[test]
fn an_unreadable_table_gives_back_nothing() {
    let mut mem = memory();
    mem.remove(&0x2000);
    let read = |p: u64| mem.get(&p).map(|t| t.as_slice());
    assert_eq!(table_frames(0x1000, 3, &read, &child), None);
}

#[test]
fn a_one_level_table_is_only_its_root() {
    let mem = memory();
    let read = |p: u64| mem.get(&p).map(|t| t.as_slice());
    assert_eq!(table_frames(0x4000, 1, &read, &child), Some(vec![0x4000]));
}
