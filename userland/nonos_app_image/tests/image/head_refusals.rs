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

//! Every head the format does not describe is refused whole, by the reason
//! the parser gives.

use super::common;

use common::{fixture, image, parts, tar};
use nonos_app_image::{build, Codec, Head, Part, Refused, HEADER_LEN, MEMBER_LEN};

fn with(f: impl Fn(&mut Vec<u8>)) -> Result<usize, Refused> {
    let mut b = image();
    f(&mut b);
    Head::parse(&b).map(|h| h.members.len())
}

#[test]
fn magic_version_and_reserved_bytes() {
    assert_eq!(with(|b| b[0] = b'X'), Err(Refused::Magic));
    assert_eq!(with(|b| b[8] = 2), Err(Refused::Magic));
    assert_eq!(with(|b| b[10] = 1), Err(Refused::Magic));
    assert_eq!(with(|b| b[14] = 1), Err(Refused::Magic));
    assert_eq!(with(|b| b[20] = 1), Err(Refused::Magic));
    assert_eq!(with(|b| b[40] = 1), Err(Refused::Magic));
    assert_eq!(with(|b| b[HEADER_LEN + 25] = 1), Err(Refused::Magic));
    assert_eq!(Head::parse(&[]).map(|_| ()), Err(Refused::Magic));
}

#[test]
fn lengths_that_do_not_tile_the_image() {
    // One byte more or less in the image, a total that names more or less.
    assert_eq!(with(|b| b.push(0)), Ok(3), "trailing bytes are not the head's to judge");
    assert_eq!(with(|b| b[24] = b[24].wrapping_add(1)), Err(Refused::Layout));
    assert_eq!(with(|b| b[24] = b[24].wrapping_sub(1)), Err(Refused::Layout));
    // A stored length one more than its bytes runs past the total.
    assert_eq!(with(|b| b[HEADER_LEN] += 1), Err(Refused::Layout));
    // A raw length that is no tar.
    assert_eq!(with(|b| b[HEADER_LEN + MEMBER_LEN + 8] += 1), Err(Refused::Layout));
    // Count zero leaves names nothing may hold; names shorter than one byte each.
    assert_eq!(with(|b| b[12] = 0), Err(Refused::TooLarge));
    assert_eq!(with(|b| b[16] = 2), Err(Refused::Layout));
}

#[test]
fn bounds_past_the_format() {
    assert_eq!(with(|b| b[13] = 0x10), Err(Refused::TooLarge), "past MAX_MEMBERS");
    assert_eq!(with(|b| b[31] = 1), Err(Refused::TooLarge), "past MAX_IMAGE");
    assert_eq!(with(|b| b[HEADER_LEN + 12] = 1), Err(Refused::TooLarge), "past MAX_RAW");
    assert_eq!(with(|b| b[19] = 0x40), Err(Refused::TooLarge), "names longer than NAME_MAX each");
}

#[test]
fn codecs_roles_and_names() {
    assert_eq!(with(|b| b[HEADER_LEN + 18] = 2), Err(Refused::Member));
    assert_eq!(with(|b| b[HEADER_LEN + 19] = 0), Err(Refused::Member), "first not listed");
    assert_eq!(with(|b| b[HEADER_LEN + MEMBER_LEN + 19] = 1), Err(Refused::Member), "two listed");
    assert_eq!(with(|b| b[HEADER_LEN + 19] = 2), Err(Refused::Member));
    let names = HEADER_LEN + 3 * MEMBER_LEN;
    assert_eq!(with(|b| b[names] = b'T'), Ok(3), "Alpine has upper case names");
    assert_eq!(with(|b| b[names] = b' '), Err(Refused::Name));
    assert_eq!(with(|b| b[names] = b'/'), Err(Refused::Name));
    assert_eq!(with(|b| b[names] = b'-'), Err(Refused::Name));
    // A stored member's length must be its tar's.
    assert_eq!(with(|b| b[HEADER_LEN + MEMBER_LEN * 2 + 18] = 0), Ok(3));
    assert_eq!(with(|b| b[HEADER_LEN + MEMBER_LEN + 18] = 0), Err(Refused::Layout));
}

#[test]
fn a_name_twice_is_refused_by_the_builder_and_the_parser() {
    let t = tar(9, 1);
    let twice = [
        Part { name: b"a", codec: Codec::Tar, tar: &t, stored: &t },
        Part { name: b"a", codec: Codec::Tar, tar: &t, stored: &t },
    ];
    assert_eq!(build(&twice).map(|_| ()), Err(Refused::Build));
    let other = [
        Part { name: b"a", codec: Codec::Tar, tar: &t, stored: &t },
        Part { name: b"b", codec: Codec::Tar, tar: &t, stored: &t },
    ];
    let mut b = build(&other).expect("built");
    let at = HEADER_LEN + 2 * MEMBER_LEN + 1;
    b[at] = b'a';
    assert_eq!(Head::parse(&b).map(|_| ()), Err(Refused::Name));
}

#[test]
fn the_builder_refuses_what_the_parser_would() {
    let (f, t) = (fixture(), tar(4, 1));
    let bad_name = [Part { name: b"Q t", codec: Codec::Tar, tar: &t, stored: &t }];
    let half_block = [Part { name: b"a", codec: Codec::Tar, tar: &t[..100], stored: &t[..100] }];
    let not_itself = [Part { name: b"a", codec: Codec::Tar, tar: &t, stored: &f.xz }];
    let empty: [Part; 0] = [];
    for parts in [&bad_name[..], &half_block, &not_itself, &empty] {
        assert_eq!(build(parts).map(|_| ()), Err(Refused::Build));
    }
    assert!(build(&parts(&f)).is_ok());
}
