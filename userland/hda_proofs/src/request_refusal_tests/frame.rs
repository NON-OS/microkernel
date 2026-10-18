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

//! A frame as a client puts it in the inbox, and what must hold for any
//! frame the driver is handed.

use crate::protocol::{
    decode_request, encode_response_header, refused, write_status, E_INVAL, HDR_LEN, STATUS_LEN,
};

const MAGIC: u32 = 0x4e48_4441;
const VERSION: u16 = 1;
/// The longest body the driver's receive buffer holds after the header.
pub(super) const BODY_MAX: usize = crate::protocol::MAX_PCM_CHUNK;
/// The last op the driver knows.
pub(super) const LAST_OP: u16 = crate::protocol::OP_OUTPUT_STATUS;

pub(super) fn frame(op: u16, request_id: u32, payload_len: u32, body: &[u8]) -> Vec<u8> {
    let mut f = Vec::with_capacity(HDR_LEN + body.len());
    f.extend_from_slice(&MAGIC.to_le_bytes());
    f.extend_from_slice(&VERSION.to_le_bytes());
    f.extend_from_slice(&op.to_le_bytes());
    f.extend_from_slice(&[0, 0, 0, 0]);
    f.extend_from_slice(&request_id.to_le_bytes());
    f.extend_from_slice(&payload_len.to_le_bytes());
    f.extend_from_slice(body);
    f
}

fn le16(b: &[u8], off: usize) -> u16 {
    u16::from_le_bytes([b[off], b[off + 1]])
}

fn le32(b: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]])
}

/// What must hold for any input. Returns whether it decoded.
pub(super) fn check(buf: &[u8]) -> bool {
    if let Some(req) = decode_request(buf) {
        assert!(buf.len() >= HDR_LEN && le32(buf, 0) == MAGIC && le16(buf, 4) == VERSION);
        assert_eq!(
            (req.op, req.request_id, req.payload_len),
            (le16(buf, 6), le32(buf, 12), le32(buf, 16))
        );
        return true;
    }
    let req = refused(buf);
    let echo = if buf.len() >= HDR_LEN { (le16(buf, 6), le32(buf, 12)) } else { (0, 0) };
    assert_eq!((req.op, req.request_id), echo, "the refusal names its request");
    let mut tx = vec![0u8; HDR_LEN + BODY_MAX];
    encode_response_header(&mut tx, &req, STATUS_LEN as u32);
    write_status(&mut tx[HDR_LEN..], E_INVAL);
    let reply = &tx[..HDR_LEN + STATUS_LEN];
    assert_eq!((le32(reply, 0), le16(reply, 4), le16(reply, 6)), (MAGIC, VERSION, req.op));
    assert_eq!((le32(reply, 12), le32(reply, 16)), (req.request_id, STATUS_LEN as u32));
    assert_eq!(le32(reply, HDR_LEN) as i32, E_INVAL);
    false
}
