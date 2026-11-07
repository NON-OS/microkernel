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

//! A net.sockets reply is sliced no further than the receive buffer, and
//! a reply shorter than its header, or a failed call, is no reply.

#[path = "../../../capsule_linux/src/linux/net/reply.rs"]
mod reply;

use reply::reply_parts;

const HDR: usize = 20;

fn rx(payload: &[u8]) -> Vec<u8> {
    let mut v = vec![0u8; HDR];
    v[8..10].copy_from_slice(&0x0102u16.to_le_bytes());
    v.extend_from_slice(payload);
    v
}

#[test]
fn a_whole_reply_gives_its_status_and_payload() {
    let b = rx(b"abcd");
    assert_eq!(reply_parts(&b, (HDR + 4) as i64, HDR), Some((0x0102, &b"abcd"[..])));
}

#[test]
fn a_short_reply_gives_only_the_bytes_reported() {
    let b = rx(b"abcd");
    assert_eq!(reply_parts(&b, (HDR + 2) as i64, HDR), Some((0x0102, &b"ab"[..])));
    assert_eq!(reply_parts(&b, HDR as i64, HDR), Some((0x0102, &b""[..])));
}

#[test]
fn a_length_past_the_buffer_is_cut_at_the_buffer() {
    let b = rx(b"abcd");
    for n in [HDR as i64 + 5, 4096, i64::MAX] {
        assert_eq!(reply_parts(&b, n, HDR), Some((0x0102, &b"abcd"[..])));
    }
}

#[test]
fn an_error_or_a_short_header_is_no_reply() {
    let b = rx(b"abcd");
    for n in [-1, -22, i64::MIN, 0, HDR as i64 - 1] {
        assert_eq!(reply_parts(&b, n, HDR), None);
    }
    assert_eq!(reply_parts(&b[..HDR - 1], HDR as i64, HDR), None);
    assert_eq!(reply_parts(&b, 9, 9), None);
}

#[test]
fn every_length_stays_inside_the_buffer() {
    let b = rx(&[7u8; 16]);
    for n in -4i64..200 {
        if let Some((_, body)) = reply_parts(&b, n, HDR) {
            assert!(HDR + body.len() <= b.len());
            assert_eq!(HDR + body.len(), (n as usize).min(b.len()));
        }
    }
}
