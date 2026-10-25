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

//! Request frames for the load handler and the replies they are checked against.

use crate::fixtures::NAME;
use crate::handlers::load_by_name::load_by_name;
use crate::protocol::{decode_request, EINVAL, OP_LOAD_BY_NAME};

pub const SEQ: u32 = 0x5EC0_0001;

/// seq | op | 2 spare bytes | payload, as decode_request reads it.
pub fn frame(op: u16, payload: &[u8]) -> Vec<u8> {
    let mut f = SEQ.to_le_bytes().to_vec();
    f.extend_from_slice(&op.to_le_bytes());
    f.extend_from_slice(&[0, 0]);
    f.extend_from_slice(payload);
    f
}

pub fn reply(status: i32, body: &[u8]) -> Vec<u8> {
    [&SEQ.to_le_bytes()[..], &status.to_le_bytes(), body].concat()
}

pub fn einval() -> Vec<u8> {
    reply(EINVAL, &[])
}

pub fn by_name(f: &[u8], sender_pid: u32) -> Vec<u8> {
    let req = decode_request(f).expect("frame decodes");
    assert_eq!(req.op, OP_LOAD_BY_NAME);
    load_by_name(req, sender_pid)
}

/// caps | name_len | NAME | args, the load_by_name payload.
pub fn by_name_payload(caps: u64, args: &[u8]) -> Vec<u8> {
    [&caps.to_le_bytes()[..], &[NAME.len() as u8], NAME, args].concat()
}
