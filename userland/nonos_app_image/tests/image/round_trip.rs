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

//! What the builder writes, the parser reads back: every member by name, in
//! order, its role, codec, place and digest, the members tiling the image.

use super::common;

use common::{fixture, image, parts};
use nonos_app_image::{head_len, Codec, Head, Role, HEADER_LEN};

#[test]
fn every_member_reads_back_as_it_went_in() {
    let (bytes, f) = (image(), fixture());
    let head = Head::parse(&bytes).expect("parsed");
    let want = parts(&f);
    assert_eq!(head.total, bytes.len() as u64);
    assert_eq!(head.members.len(), want.len());
    for (m, p) in head.members.iter().zip(&want) {
        assert_eq!((m.name, m.codec), (p.name, p.codec));
        assert_eq!(&bytes[m.at.start as usize..m.at.end as usize], p.stored);
        assert!(m.holds(p.tar), "{:?}", m.name);
    }
    assert_eq!(head.listed().name, b"telegram-desktop");
    assert_eq!(head.members.iter().filter(|m| m.role == Role::Listed).count(), 1);
    assert_eq!(head.members[1].codec, Codec::Xz);
}

#[test]
fn the_head_alone_is_enough_to_read_the_table() {
    let bytes = image();
    let n = head_len(&bytes[..HEADER_LEN]).expect("head length");
    let head = Head::parse(&bytes[..n]).expect("head alone");
    assert_eq!(head.members.len(), 3);
    assert!(Head::parse(&bytes[..n - 1]).is_err(), "one byte short of the names");
}

#[test]
fn a_member_holds_only_its_own_tar() {
    let (bytes, f) = (image(), fixture());
    let head = Head::parse(&bytes).expect("parsed");
    assert!(!head.members[0].holds(&f.tars[1]));
    let mut other = f.tars[0].clone();
    other[7] ^= 1;
    assert!(!head.members[0].holds(&other));
    assert!(!head.members[0].holds(&f.tars[0][..512]));
}

#[test]
fn the_same_members_build_the_same_bytes() {
    let f = fixture();
    let a = nonos_app_image::build(&parts(&f)).expect("built");
    let b = nonos_app_image::build(&parts(&f)).expect("built");
    assert_eq!(a, b);
}
