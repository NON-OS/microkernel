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

//! A package is gzip members back to back. Each is sized from the last
//! member's ISIZE, which is the data's; a small member must give that room
//! back, or each holds a block the size of the whole package. ISIZE is the
//! sender's word, so a lie either way is refused by the trailer check.

use nonos_inflate::{gunzip_partial, members_checked, End, Refused};

const APK: &[u8] = include_bytes!("vectors/apk_shape.gz");
const MORE: &[u8] = include_bytes!("vectors/isize_more.gz");
const LESS: &[u8] = include_bytes!("vectors/isize_less.gz");
const MIB: usize = 1 << 20;

#[test]
fn a_small_member_keeps_no_room_for_the_data() {
    let m = members_checked(APK, 4 * MIB).expect("three members");
    let bodies: Vec<&[u8]> = m.iter().map(|m| m.body.as_slice()).collect();
    assert_eq!((bodies[0], bodies[1], bodies[2].len()), (&b"sig"[..], &b"control"[..], MIB));
    for small in &m[..2] {
        assert!(small.body.capacity() <= 1 << 16, "a small member holds {}", small.body.capacity());
    }
}

#[test]
fn an_isize_lie_either_way_is_refused() {
    for lie in [MORE, LESS] {
        assert_eq!(members_checked(lie, 4 * MIB).map(|m| m.len()), Err(Refused::Bad));
        assert_eq!(gunzip_partial(lie, 4 * MIB).end, End::Corrupt);
    }
}
