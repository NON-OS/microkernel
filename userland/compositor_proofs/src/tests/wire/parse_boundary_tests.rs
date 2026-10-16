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

//! The frames at every boundary of the header decode, each served or
//! refused with its reply.

use super::parse_check::*;
use super::parse_frames::*;
use crate::protocol::{
    parse, E_BAD_LEN, E_BAD_MAGIC, E_BAD_VERSION, HDR_LEN, IPC_PAYLOAD_MAX, OP_DISPLAY_INFO,
    OP_HEALTHCHECK, OP_SCENE_SUBMIT,
};

#[test]
fn boundary_frames() {
    // Empty, and one byte short of a header: refused under zeros.
    check(&[]);
    assert_eq!(parse(&[]).err().map(|(c, _)| c), Some(E_BAD_LEN));
    let short = frame(OP_HEALTHCHECK, 0, 7, 0, &[]);
    check(&short[..HDR_LEN - 1]);
    assert_eq!(parse(&short[..HDR_LEN - 1]).err().map(|(c, _)| c), Some(E_BAD_LEN));
    // A bare header with an empty body is served.
    assert!(parse(&short).is_ok());
    // A length field larger, and smaller, than the bytes that came.
    for (len_field, body) in [(5u32, &[1u8, 2, 3, 4][..]), (3, &[1, 2, 3, 4][..])] {
        let f = frame(OP_HEALTHCHECK, 0, 9, len_field, body);
        check(&f);
        let (code, req) = parse(&f).err().expect("a length that disagrees is refused");
        assert_eq!((code, req.request_id), (E_BAD_LEN, 9));
    }
    // The length field at its maximum does not wrap into a short frame.
    let f = frame(OP_HEALTHCHECK, 0, 11, u32::MAX, &[]);
    check(&f);
    assert_eq!(parse(&f).err().map(|(c, _)| c), Some(E_BAD_LEN));
    // Every op the compositor knows, one past the last, and the largest.
    for op in (0..=OP_DISPLAY_INFO + 1).chain([u16::MAX]) {
        let f = frame(op, 3, u32::from(op) + 100, 0, &[]);
        check(&f);
        let (req, body) = parse(&f).ok().expect("a well formed frame is served");
        assert_eq!((req.op, req.request_id, body.len()), (op, u32::from(op) + 100, 0));
    }
    // The largest body the receive buffer holds is served whole.
    let big = vec![0xA5u8; IPC_PAYLOAD_MAX];
    let f = frame(OP_SCENE_SUBMIT, 0, 13, IPC_PAYLOAD_MAX as u32, &big);
    check(&f);
    assert_eq!(parse(&f).ok().map(|(_, b)| b.len()), Some(IPC_PAYLOAD_MAX));
    // Wrong magic and wrong version echo the request they refuse.
    let mut f = frame(OP_HEALTHCHECK, 0, 15, 0, &[]);
    f[0] ^= 0xFF;
    assert_eq!(parse(&f).err().map(|(c, r)| (c, r.request_id)), Some((E_BAD_MAGIC, 15)));
    let mut f = frame(OP_HEALTHCHECK, 0, 17, 0, &[]);
    f[4] = 2;
    assert_eq!(parse(&f).err().map(|(c, r)| (c, r.request_id)), Some((E_BAD_VERSION, 17)));
}
