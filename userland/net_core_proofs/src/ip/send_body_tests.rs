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

//! A net.ip send body is read only when it holds the address, the protocol
//! and a whole ICMP header: every shorter body, which once indexed past
//! its end and ended net.core, is answered E_BAD_LEN, as is one past
//! MAX_PAYLOAD; another protocol or identifier 0 is E_BAD_PROTO.

use super::send_body::{read, Send, ICMP_HDR};
use crate::protocol::ip::{E_BAD_LEN, E_BAD_PROTO, MAX_PAYLOAD, PROTO_ICMP};

/// A well-formed echo body of `len` bytes, or the first `len` bytes of one.
fn body(len: usize) -> Vec<u8> {
    let mut b = vec![0u8; len.max(5 + ICMP_HDR)];
    b[..5].copy_from_slice(&[10, 0, 2, 2, PROTO_ICMP]);
    b[5] = 8;
    b[9..11].copy_from_slice(&0x4E53u16.to_be_bytes());
    b.truncate(len);
    b
}

#[test]
fn every_body_shorter_than_an_icmp_header_is_refused() {
    for len in 0..5 + ICMP_HDR {
        assert_eq!(read(&body(len)), Err(E_BAD_LEN), "{len}");
    }
}

#[test]
fn a_whole_header_is_read() {
    let b = body(5 + ICMP_HDR);
    assert_eq!(read(&b), Ok(Send { dst: [10, 0, 2, 2], ident: 0x4E53, message: &b[5..] }));
    let longest = body(5 + MAX_PAYLOAD);
    assert_eq!(read(&longest).map(|s| s.message.len()), Ok(MAX_PAYLOAD));
}

#[test]
fn a_body_past_the_bound_is_refused() {
    assert_eq!(read(&body(5 + MAX_PAYLOAD + 1)), Err(E_BAD_LEN));
}

#[test]
fn another_protocol_or_identifier_zero_is_refused() {
    let mut udp = body(5 + ICMP_HDR);
    udp[4] = 17;
    assert_eq!(read(&udp), Err(E_BAD_PROTO));
    let mut zero = body(5 + ICMP_HDR);
    zero[9..11].fill(0);
    assert_eq!(read(&zero), Err(E_BAD_PROTO));
}

#[test]
fn no_body_of_any_length_or_bytes_panics() {
    let mut seed = 0x9E37_79B9_7F4A_7C15u64;
    for len in 0..5 + MAX_PAYLOAD + 8 {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        let b: Vec<u8> = (0..len).map(|i| (seed >> (i % 8 * 8)) as u8).collect();
        let _ = read(&b);
    }
}
