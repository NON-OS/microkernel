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

//! Replies go to the driver's reply endpoint, which the kernel's client
//! drains and the kernel's call primitive hands to a capsule caller.

use nonos_libc::mk_ipc_send;

use super::endpoint::REPLY_ENDPOINT;
use crate::wire::{encode_response_header, write_status, Request, RESP_HDR_LEN, STATUS_LEN};

/// A status and nothing after it.
pub fn status(tx: &mut [u8], req: &Request, status: i32) {
    encode_response_header(tx, req, STATUS_LEN as u32);
    write_status(&mut tx[RESP_HDR_LEN..], status);
    let _ = mk_ipc_send(REPLY_ENDPOINT, tx.as_ptr(), RESP_HDR_LEN + STATUS_LEN);
}

/// Success, and the `len` bytes the handler wrote after the status word.
pub fn body(tx: &mut [u8], req: &Request, len: usize) {
    encode_response_header(tx, req, (STATUS_LEN + len) as u32);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    let _ = mk_ipc_send(REPLY_ENDPOINT, tx.as_ptr(), RESP_HDR_LEN + STATUS_LEN + len);
}

/// Where a handler writes its reply body.
pub const BODY_AT: usize = RESP_HDR_LEN + STATUS_LEN;
