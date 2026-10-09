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

//! The frames on the wire, byte for byte as both proxies parse them
//! (capsule_socks5 server/request.rs, capsule_net_anon server/socks/frame.rs).

use crate::frame::{next_seq, numbered, reset, CARRY_MAX, FIRST_SEQ, HEAD, NUMBERED, RESET};

#[test]
fn a_reset_is_one_byte() {
    assert_eq!(reset(), [1]);
    assert_eq!(RESET, 1);
}

#[test]
fn a_numbered_frame_is_marker_number_bytes() {
    let f = numbered(0x0403_0201, b"hi").unwrap_or_default();
    assert_eq!(f, [NUMBERED, 1, 2, 3, 4, b'h', b'i']);
    /* A poll is the head alone, never a single byte the proxy reads as
     * the old unnumbered shape. */
    let poll = numbered(FIRST_SEQ, &[]).unwrap_or_default();
    assert_eq!(poll, [2, 1, 0, 0, 0]);
    assert_eq!(poll.len(), HEAD);
}

#[test]
fn a_frame_carries_at_most_one_record_and_its_overhead() {
    let full = vec![7u8; CARRY_MAX];
    assert_eq!(numbered(9, &full).map(|f| f.len()), Some(HEAD + CARRY_MAX));
    assert_eq!(numbered(9, &[7u8; CARRY_MAX + 1]), None);
}

/* Both proxies read a request into 32 KiB or more. */
const _: () = assert!(HEAD + CARRY_MAX <= 32 * 1024);

#[test]
fn numbers_move_on_and_skip_zero() {
    assert_eq!(FIRST_SEQ, 1);
    assert_eq!(next_seq(1), 2);
    assert_eq!(next_seq(u32::MAX), 1);
    assert_eq!(next_seq(0), 1);
    for s in [1u32, 2, 77, u32::MAX - 1, u32::MAX] {
        assert_ne!(next_seq(s), s);
        assert_ne!(next_seq(s), 0);
    }
}
