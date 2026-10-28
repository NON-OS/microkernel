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

//! The codec refuses a bad magic, a `len` that is not the body, a body past
//! the bound, an unknown op, and a question its op may not carry (P.9).

use crate::*;

pub(super) fn frame(op: u8, status: u8, conn: u16, body: &[u8]) -> Vec<u8> {
    let mut out = vec![0u8; HEADER + body.len()];
    let n = put(&mut out, Header { op, status, conn, len: 0 }, body).unwrap();
    out.truncate(n);
    out
}

#[test]
fn a_frame_reads_back_as_written_with_its_numbers_on_the_wire() {
    let f = frame(WRITE, 0, 7, b"hello");
    assert_eq!(&f[..12], &[b'N', b'S', b'O', b'1', 2, 0, 7, 0, 5, 0, 0, 0]);
    assert_eq!(decode(&f), Ok((Header { op: WRITE, status: 0, conn: 7, len: 5 }, &b"hello"[..])));
    let a = frame(READ, CLOSED, 7, b"");
    assert_eq!(decode(&a).unwrap().0.status, CLOSED);
}

#[test]
fn len_must_name_exactly_the_bytes_after_the_header() {
    let mut f = frame(WRITE, 0, 1, b"abc");
    f.push(b'x');
    assert_eq!(decode(&f), Err(Refused::Len), "a byte past len");
    assert_eq!(decode(&f[..f.len() - 2]), Err(Refused::Len), "a byte short of len");
    assert_eq!(decode(&f[..11]), Err(Refused::Short));
    let mut big = frame(WRITE, 0, 1, b"");
    big[8..12].copy_from_slice(&((MAX_BODY as u32) + 1).to_le_bytes());
    big.resize(HEADER + MAX_BODY + 1, 0);
    assert_eq!(decode(&big), Err(Refused::Len), "past MAX_BODY");
    assert!(put(
        &mut vec![0u8; HEADER + MAX_BODY + 1],
        Header { op: WRITE, status: 0, conn: 1, len: 0 },
        &vec![0u8; MAX_BODY + 1]
    )
    .is_none());
}

#[test]
fn a_bad_magic_or_op_is_refused() {
    let mut f = frame(OPEN, 0, 0, b"");
    f[3] = b'2';
    assert_eq!(decode(&f), Err(Refused::Magic));
    for op in [0u8, 5, 255] {
        assert_eq!(decode(&frame(op, 0, 0, b"")), Err(Refused::Op), "op {op}");
    }
}

mod question;
