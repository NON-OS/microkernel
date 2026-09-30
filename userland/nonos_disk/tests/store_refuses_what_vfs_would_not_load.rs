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

//! Names, counts and sizes the vfs would refuse are refused before any
//! write, and a group of files that cannot go in whole adds nothing. What
//! did go in is read back by the vfs's own decoding.

#[path = "common/vfs/mod.rs"]
mod vfs;

/*
 * The vfs files name `alloc`, as a no_std capsule's do.
 */
extern crate alloc;

use nonos_disk::{StoreBuilder, StoreError};
use nonos_disk_map::{MAX_ENTRIES, MAX_TOTAL_BYTES, NAME_LEN};

#[test]
fn the_store_refuses_what_vfs_would_not_load() {
    let mut store = StoreBuilder::new();
    assert_eq!(store.add("", b"x"), Err(StoreError::BadName));
    assert_eq!(store.add(&"/".repeat(NAME_LEN + 1), b"x"), Err(StoreError::BadName));
    assert_eq!(store.add("/big", &vec![0; MAX_TOTAL_BYTES as usize + 1]), Err(StoreError::Full));
    store.add("/a", b"x").unwrap();
    assert_eq!(store.add("/a", b"y"), Err(StoreError::Duplicate));
    let group: [(&str, &[u8]); 2] = [("/b", b"1"), ("/a", b"2")];
    assert_eq!(store.add_all(&group), Err(StoreError::Duplicate));
    for i in 1..MAX_ENTRIES {
        store.add(&format!("/f{i}"), b"z").unwrap();
    }
    assert_eq!(store.add("/one-more", b"z"), Err(StoreError::Full));
    let names: Vec<String> =
        vfs::load(store.finish().bytes()).into_iter().map(|(n, _)| n).collect();
    assert_eq!(names.len(), MAX_ENTRIES);
    assert!(!names.contains(&"/b".to_string()), "a group that failed added nothing");
}
