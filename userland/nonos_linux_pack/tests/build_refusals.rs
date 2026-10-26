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

//! What the builder will not write: anything the parser would refuse, so a
//! package that cannot be a pack fails its install before any of it lands.

mod common;

use common::{src, vimish};
use nonos_linux_pack::{build, Kind, Refused, MAX_PACK};

#[test]
fn a_name_twice_a_bad_name_or_a_bad_link_is_no_pack() {
    let twice = || vec![src("a", Kind::File, false, b"1"), src("a", Kind::File, false, b"2")];
    assert_eq!(build(twice()).err(), Some(Refused::Build));
    for name in ["", "/etc/passwd", "a/../b", "a//b", "./a", "a/"] {
        let one = vec![src(name, Kind::File, false, b"x")];
        assert_eq!(build(one).err(), Some(Refused::Build), "{name:?}");
    }
    for (kind, bytes) in [(Kind::Link, &b""[..]), (Kind::Link, b"a\0b"), (Kind::Dir, b"x")] {
        assert_eq!(build(vec![src("a", kind, false, bytes)]).err(), Some(Refused::Build));
    }
}

#[test]
fn a_package_past_the_largest_pack_is_refused_not_cut() {
    let body = vec![0u8; MAX_PACK];
    let big = vec![src("big", Kind::File, false, &body)];
    assert_eq!(build(big).err(), Some(Refused::Build));
}

#[test]
fn the_order_sources_come_in_is_not_the_packs() {
    let mut reversed = vimish();
    reversed.reverse();
    assert_eq!(build(reversed).unwrap(), build(vimish()).unwrap());
}

#[test]
fn an_empty_pack_is_a_pack() {
    let bytes = build(Vec::new()).unwrap();
    assert!(nonos_linux_pack::Pack::parse(&bytes).unwrap().is_empty());
}
