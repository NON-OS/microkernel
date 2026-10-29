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

//! What one package costs the installer: its inflated tar and its pack,
//! about twice its installed size, never a third copy of its files (the tar
//! walk borrows them), asked before it is inflated (fetch/room.rs), in a
//! heap twice the old one where memory is to spare (heap.rs). Telegram's
//! install ended on an 8 GB laptop with 134: three copies of a package past
//! 100 MB passed the install heap.

use super::super::tar_build::file;
use crate::install::index::Index;
use crate::install::tar::walk;

#[path = "../../../../capsule_linux/src/linux/install/fetch/room.rs"]
mod room;

const HEAP: &str = include_str!("../../../../capsule_linux/src/linux/heap.rs");

#[test]
fn every_member_body_is_a_slice_of_the_package_not_a_copy() {
    let data = [file("usr/bin/a", &[7u8; 3000]), file("usr/lib/b", b"bee")].concat();
    let (w, all) = (walk(&data), data.as_ptr_range());
    assert_eq!(w.entries.len(), 2);
    for e in &w.entries {
        let body = e.body.as_ptr_range();
        assert!(all.start <= body.start && body.end <= all.end, "a copy");
    }
}

#[test]
fn a_package_is_placed_in_about_twice_its_installed_size() {
    let telegram = 110 << 20;
    assert_eq!(room::needs(0), 0, "no size given: the fallible inflate bounds it");
    assert_eq!(room::needs(telegram), 2 * telegram + telegram / 8);
    assert_eq!(room::needs(1024), 2048 + (1 << 20));
    assert_eq!(room::needs(usize::MAX), usize::MAX);
    assert!(room::needs(telegram) <= 320 << 20, "Telegram's in the old heap");
}

#[test]
fn the_index_gives_each_package_its_installed_size() {
    let mut index = Index::default();
    index.add(b"P:telegram-desktop\nV:5.0.1-r6\nS:36000000\nI:115000000\n\n", 0);
    let p = index.by_name("telegram-desktop").expect("listed");
    assert_eq!((p.size, p.installed), (36_000_000, 115_000_000));
}

#[test]
fn an_install_takes_twice_its_heap_with_memory_to_spare_then_less() {
    assert!(HEAP.contains("let sizes = [INSTALL_HEAP * 2, INSTALL_HEAP, INSTALL_HEAP / 2];"));
    assert!(HEAP.contains(".skip(usize::from(!spare())).any(|&s| heap_init_sized(s).is_ok())"));
}

#[test]
fn room_is_asked_before_the_package_is_inflated() {
    let get = include_str!("../../../../capsule_linux/src/linux/install/fetch/get.rs");
    let asked = get.find("super::room::needs(pkg.installed)").expect("asked");
    assert!(asked < get.find("verified(&apk, &s)").expect("inflated"));
    assert!(get.contains("room.and(pkg.checksum"), "a refusal is UNPACK_MEMORY, named");
}
