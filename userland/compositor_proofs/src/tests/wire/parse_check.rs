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

//! The one check every frame goes through: decoded without a panic, and
//! either served or refused with exactly the reply the drain sends.

use super::parse_frames::*;
use crate::protocol::{
    parse, response_header, write_status, Request, E_BAD_LEN, E_BAD_MAGIC, E_BAD_VERSION, HDR_LEN,
    IPC_PAYLOAD_MAX, MAGIC, STATUS_LEN, VERSION,
};

/// The reply the drain sends for a refusal, built by the encoders it uses.
pub(crate) fn refusal_reply(req: &Request, code: i32) -> Vec<u8> {
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    response_header(&mut tx, req, STATUS_LEN as u32);
    write_status(&mut tx, code);
    tx.truncate(HDR_LEN + STATUS_LEN);
    tx
}

/// What must hold for any input, whatever it is.
pub(crate) fn check(buf: &[u8]) {
    match parse(buf) {
        Ok((req, body)) => {
            assert!(buf.len() >= HDR_LEN);
            assert_eq!(le32(buf, 0), MAGIC);
            assert_eq!(le16(buf, 4), VERSION);
            assert_eq!(req.op, le16(buf, 6));
            assert_eq!(req.flags, le16(buf, 8));
            assert_eq!(req.request_id, le32(buf, 12));
            assert_eq!(body.len() as u64, u64::from(le32(buf, 16)));
            assert_eq!(body, &buf[HDR_LEN..], "the body is the rest of the frame");
        }
        Err((code, req)) => {
            assert!([E_BAD_LEN, E_BAD_MAGIC, E_BAD_VERSION].contains(&code), "status {code}");
            if buf.len() < HDR_LEN {
                assert_eq!(code, E_BAD_LEN);
                assert_eq!((req.op, req.flags, req.request_id), (0, 0, 0));
            } else {
                assert_eq!(
                    (req.op, req.flags, req.request_id),
                    (le16(buf, 6), le16(buf, 8), le32(buf, 12))
                );
            }
            let reply = refusal_reply(&req, code);
            assert_eq!(reply.len(), HDR_LEN + STATUS_LEN);
            assert_eq!(le32(&reply, 0), MAGIC);
            assert_eq!(le16(&reply, 4), VERSION);
            assert_eq!(le16(&reply, 6), req.op);
            assert_eq!(le32(&reply, 12), req.request_id);
            assert_eq!(le32(&reply, 16), STATUS_LEN as u32);
            assert_eq!(le32(&reply, HDR_LEN) as i32, code);
        }
    }
}
