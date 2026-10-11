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

//! Sending a reply: a status alone, or a success carrying a payload.

use nonos_libc::mk_ipc_send;

use crate::protocol::{
    encode_response_header, write_status, Request, KERNEL_REPLY_ENDPOINT, RESP_HDR_LEN, STATUS_LEN,
};

pub fn status(tx: &mut [u8], req: &Request, status: i32) {
    encode_response_header(tx, req, STATUS_LEN as u32);
    write_status(&mut tx[RESP_HDR_LEN..], status);
    let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), RESP_HDR_LEN + STATUS_LEN);
}

/// A header that did not decode is answered with a request of zeros.
pub fn decode_failed(tx: &mut [u8], s: i32) {
    let req = Request { op: 0, flags: 0, unit: 0, request_id: 0, payload_len: 0 };
    status(tx, &req, s);
}

/// A success reply carrying `payload` after the status word.
pub fn body(tx: &mut [u8], req: &Request, payload: &[u8]) {
    let at = RESP_HDR_LEN + STATUS_LEN;
    tx[at..at + payload.len()].copy_from_slice(payload);
    finish(tx, req, payload.len());
}

/// A success reply whose `len` payload bytes already sit after the status.
pub fn finish(tx: &mut [u8], req: &Request, len: usize) {
    encode_response_header(tx, req, (STATUS_LEN + len) as u32);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), RESP_HDR_LEN + STATUS_LEN + len);
}
