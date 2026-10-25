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

//! The power-off request's bytes, and every way an answer is read: none of
//! them is taken for a power off, because one that works never answers.

use crate::power_off_frame::{heard, request, Heard, HDR_LEN, MAGIC, OP_OFF, SERVICE, VERSION};

fn reply(magic: u32, id: u32, status: i32) -> [u8; HDR_LEN + 4] {
    let mut r = [0u8; HDR_LEN + 4];
    r[..HDR_LEN].copy_from_slice(&request(id));
    r[0..4].copy_from_slice(&magic.to_le_bytes());
    r[HDR_LEN..].copy_from_slice(&status.to_le_bytes());
    r
}

#[test]
fn the_request_is_the_twenty_byte_header_with_no_payload() {
    let f = request(0x0102_0304);
    assert_eq!(&f[0..4], b"POFF");
    assert_eq!(MAGIC.to_le_bytes(), *b"POFF");
    assert_eq!(u16::from_le_bytes([f[4], f[5]]), VERSION);
    assert_eq!(u16::from_le_bytes([f[6], f[7]]), OP_OFF);
    assert_eq!(&f[8..12], &[0; 4], "flags and the reserved half-word");
    assert_eq!(&f[12..16], &[4, 3, 2, 1]);
    assert_eq!(&f[16..20], &[0; 4], "no payload");
    assert_eq!(SERVICE, b"power.off");
}

#[test]
fn an_answer_to_this_request_is_its_refusal() {
    for status in [0, -1, -13, i32::MIN, i32::MAX] {
        let r = reply(MAGIC, 9, status);
        assert_eq!(heard(r.len() as i64, &r, 9), Heard::Refused(status));
    }
}

#[test]
fn anything_else_is_garbled_or_silent() {
    let r = reply(MAGIC, 9, -1);
    assert_eq!(heard(-110, &r, 9), Heard::Silent, "timed out");
    assert_eq!(heard(i64::MIN, &r, 9), Heard::Silent);
    for n in 0..r.len() as i64 {
        assert_eq!(heard(n, &r, 9), Heard::Garbled, "{n} bytes");
    }
    assert_eq!(heard(r.len() as i64 + 1, &r, 9), Heard::Garbled, "longer than the buffer");
    assert_eq!(heard(r.len() as i64, &r, 10), Heard::Garbled, "another request's answer");
    let bad = reply(MAGIC ^ 1, 9, -1);
    assert_eq!(heard(bad.len() as i64, &bad, 9), Heard::Garbled, "not the service's magic");
}
