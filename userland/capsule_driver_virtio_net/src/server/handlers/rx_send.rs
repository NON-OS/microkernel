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

//! Sending a status-zero reply whose body does not fit the fixed buffer.

extern crate alloc;

use alloc::vec;

use crate::protocol::{encode_response_header, write_status, Request, RESP_HDR_LEN, STATUS_LEN};
use crate::server::error::reply;

/// Reply to `pid` with status 0 followed by `body`.
pub fn send(pid: u32, req: &Request, body: &[u8]) -> bool {
    let mut out = vec![0u8; RESP_HDR_LEN + STATUS_LEN + body.len()];
    encode_response_header(&mut out, req, (STATUS_LEN + body.len()) as u32);
    write_status(&mut out[RESP_HDR_LEN..], 0);
    out[RESP_HDR_LEN + STATUS_LEN..].copy_from_slice(body);
    reply(pid, &out, out.len())
}
