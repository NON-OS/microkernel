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

//! The tar walk's refusals and records (tar.rs, tar_field.rs, tar_kind.rs):
//! a size that is not octal or runs past the data ends the walk with what
//! came before; GNU long names and links and global pax records are records,
//! not entries; ustar's prefix goes in front of the name only when set.

use crate::install::tar::{walk, Kind};
use crate::install::unpacked::unpacked;

use super::tar_build::{block, file, head};

#[test]
fn a_size_that_is_not_octal_or_runs_past_the_data_ends_the_walk() {
    let first = file("usr/a", b"one");
    for bad in [head("usr/b", "9", b'0', "", None), head("usr/b", "7777", b'0', "", None)] {
        let w = walk(&[first.clone(), bad, block(b"x")].concat());
        let names: Vec<&[u8]> = w.entries.iter().map(|e| e.name.as_slice()).collect();
        assert_eq!(names, [b"usr/a"]);
    }
}

#[test]
fn gnu_long_names_and_links_name_the_next_entry_and_global_pax_is_dropped_silently() {
    let long = "usr/".to_string() + &"n".repeat(150);
    let data = [
        head("././@LongLink", &format!("{:o}", long.len()), b'L', "", None),
        block(long.as_bytes()),
        head("././@LongLink", "6", b'K', "", None),
        block(b"target"),
        head("short", "0", b'2', "ignored", None),
        head("pax_global_header", "0", b'g', "", None),
        file("usr/c", b"c"),
    ]
    .concat();
    let w = walk(&data);
    assert_eq!(w.dropped, 0);
    assert_eq!(w.entries[0].name, long.as_bytes());
    assert!(matches!(&w.entries[0].kind, Kind::Symlink(t) if t == b"target"));
    assert_eq!(w.entries[1].name, b"usr/c");
}

#[test]
fn ustar_prefix_goes_in_front_only_when_set() {
    let set =
        [head("x", "0", b'5', "", Some("usr/share")), head("y", "0", b'5', "", Some(""))].concat();
    let w = walk(&set);
    assert_eq!(w.entries[0].name, b"usr/share/x");
    assert_eq!(w.entries[1].name, b"y");
    assert_eq!(unpacked(&set).as_deref(), Some(&set[..]), "an uncompressed tar passes as it is");
}
