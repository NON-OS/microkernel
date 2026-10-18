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

//! Maps a peripheral could send to steer the host: each is refused, and
//! refused for its own reason, before any field is used.

use super::maps::{global, item, local, main, KEYBOARD};
use nonos_bt_hogp::report::map::{parse, MapError::*, DEPTH_MAX, MAP_MAX};

fn open() -> Vec<u8> {
    main(10, 1)
}

fn close() -> Vec<u8> {
    item(0, 12, &[])
}

#[test]
fn a_map_past_the_bound_or_cut_short_or_of_unknown_items() {
    // Physical Minimum with no data: an item that changes nothing.
    let long = [&KEYBOARD[..], &[0x34; MAP_MAX - 90 + 1][..]].concat();
    assert_eq!(parse(&long), Err(TooMany));
    assert!(parse(&long[..MAP_MAX]).is_ok(), "MAP_MAX octets are read");
    assert_eq!(parse(&KEYBOARD[..KEYBOARD.len() - 2]), Err(Short), "inside an item");
    assert_eq!(parse(&[0xfe, 0x01, 0x10, 0x00]), Err(Unknown), "a long item");
    assert_eq!(parse(&[0x0c | 0x01, 0x00]), Err(Unknown), "the reserved item type");
    assert_eq!(parse(&main(13, 0)), Err(Unknown), "a reserved main tag");
    assert_eq!(parse(&global(12, 0)), Err(Unknown), "a reserved global tag");
    assert_eq!(parse(&local(10, 1)), Err(Unknown), "a delimiter");
}

#[test]
fn collections_and_pushes_must_pair_within_their_bounds() {
    assert_eq!(parse(&close()), Err(Shape));
    assert_eq!(parse(&open()), Err(Shape), "left open");
    let deep = |n: usize| [open().repeat(n), close().repeat(n)].concat();
    assert!(parse(&deep(DEPTH_MAX)).is_ok());
    assert_eq!(parse(&deep(DEPTH_MAX + 1)), Err(TooMany));
    assert_eq!(parse(&item(1, 11, &[])), Err(Shape), "a Pop with nothing pushed");
    assert!(parse(&item(1, 10, &[]).repeat(4)).is_ok());
    assert_eq!(parse(&item(1, 10, &[]).repeat(5)), Err(TooMany));
}

#[test]
fn usages_must_pair_and_stay_bounded() {
    let input = || [global(7, 1), global(9, 1), main(8, 2)].concat();
    assert_eq!(parse(&[local(1, 4), input()].concat()), Err(Shape), "a Minimum alone");
    assert_eq!(parse(&[local(2, 4), input()].concat()), Err(Shape), "a Maximum alone");
    assert_eq!(parse(&[local(1, 4), local(1, 5)].concat()), Err(Shape));
    assert_eq!(parse(&[local(1, 9), local(2, 4)].concat()), Err(Value), "Maximum under Minimum");
    assert!(parse(&[local(0, 4).repeat(16), input()].concat()).is_ok());
    assert_eq!(parse(&local(0, 4).repeat(17)), Err(TooMany));
}
