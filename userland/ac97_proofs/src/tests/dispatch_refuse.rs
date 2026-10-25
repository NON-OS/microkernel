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

//! Requests the driver refuses outright.

use super::util::{ask, request, status, up};
use crate::server::Stream;

#[test]
fn ops_the_server_never_sends_are_refused() {
    let (mut d, _ich) = up();
    for op in [0, 2, 3, 4, 5, 6, 11, 0xFFFF] {
        assert_eq!(status(&ask(&mut d, &mut Stream::default(), &request(op, 1, &[]))), -22);
    }
}

#[test]
fn a_frame_that_is_not_the_wire_is_answered_under_what_it_names() {
    let (mut d, _ich) = up();
    let mut bad = request(10, 77, &[]);
    bad[0] ^= 1;
    let r = ask(&mut d, &mut Stream::default(), &bad);
    assert_eq!((&r[6..8], &r[12..16], status(&r)), (&[10u8, 0][..], &77u32.to_le_bytes()[..], -22));
    let r = ask(&mut d, &mut Stream::default(), &[0x41, 0x44]);
    assert_eq!((&r[6..8], &r[12..16], status(&r)), (&[0u8, 0][..], &[0u8; 4][..], -22));
}
