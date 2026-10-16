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

//! The reply decoder against the replies a driver sends: the header it
//! must echo, the status, and the payload after it, handed back whole.

use crate::error::BlkError;
use crate::wire::{decode_reply, HDR_LEN, STATUS_LEN, VERSION};

const MAGIC: u32 = 0x4E42_4C4B;

/// A reply as a driver encodes it: header, status, then `payload`.
pub fn reply(magic: u32, op: u16, id: u32, status: i32, payload: &[u8]) -> Vec<u8> {
    let mut r = Vec::new();
    r.extend_from_slice(&magic.to_le_bytes());
    r.extend_from_slice(&VERSION.to_le_bytes());
    r.extend_from_slice(&op.to_le_bytes());
    r.extend_from_slice(&[0; 4]);
    r.extend_from_slice(&id.to_le_bytes());
    r.extend_from_slice(&((STATUS_LEN + payload.len()) as u32).to_le_bytes());
    r.extend_from_slice(&status.to_le_bytes());
    r.extend_from_slice(payload);
    r
}

#[test]
fn a_well_formed_reply_hands_back_its_payload_whole() {
    let r = reply(MAGIC, 3, 77, 0, &[1, 2, 3, 4, 5]);
    assert_eq!(decode_reply(&r, r.len(), MAGIC, 3, 77), Ok(&[1u8, 2, 3, 4, 5][..]));
    let r = reply(MAGIC, 5, 1, 0, &[]);
    assert_eq!(decode_reply(&r, r.len(), MAGIC, 5, 1), Ok(&[][..]));
}

#[test]
fn every_way_a_reply_can_be_wrong_is_named() {
    let r = reply(MAGIC, 3, 77, 0, &[9; 8]);
    let n = r.len();
    assert_eq!(decode_reply(&r, n, MAGIC + 1, 3, 77), Err(BlkError::BadHeader));
    assert_eq!(decode_reply(&r, n, MAGIC, 4, 77), Err(BlkError::BadHeader));
    assert_eq!(decode_reply(&r, n, MAGIC, 3, 78), Err(BlkError::IdMismatch));
    let mut v = r.clone();
    v[4] ^= 1;
    assert_eq!(decode_reply(&v, n, MAGIC, 3, 77), Err(BlkError::BadHeader), "version");
    let failed = reply(MAGIC, 3, 77, -110, &[]);
    assert_eq!(decode_reply(&failed, failed.len(), MAGIC, 3, 77), Err(BlkError::Status(-110)));
    for short in 0..HDR_LEN + STATUS_LEN {
        assert_eq!(decode_reply(&r, short, MAGIC, 3, 77), Err(BlkError::ShortReply(short)));
    }
    assert_eq!(decode_reply(&r, n + 1, MAGIC, 3, 77), Err(BlkError::ShortReply(n + 1)));
    assert_eq!(decode_reply(&r, n - 1, MAGIC, 3, 77), Err(BlkError::BadLength), "payload cut");
}
