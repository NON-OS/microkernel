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

use nonos_libc::{mk_ipc_reply, mk_ipc_send};

use crate::protocol::{
    response_header, write_status, Request, HDR_LEN, KERNEL_REPLY_ENDPOINT, STATUS_LEN,
};

/// A capsule caller gets a reply matched to its call; the kernel's block
/// client, which arrives as pid 0, drains its fixed reply inbox by request id.
fn deliver(sender_pid: u32, tx: &[u8], len: usize) -> i64 {
    if sender_pid == 0 {
        mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), len)
    } else {
        mk_ipc_reply(sender_pid, tx.as_ptr(), len)
    }
}

pub fn status(sender_pid: u32, req: &Request, errno: i32, tx: &mut [u8]) -> i64 {
    response_header(tx, req, STATUS_LEN as u32);
    write_status(tx, errno);
    deliver(sender_pid, tx, HDR_LEN + STATUS_LEN)
}

pub fn payload(sender_pid: u32, req: &Request, body_len: usize, tx: &mut [u8]) -> i64 {
    let payload_len = STATUS_LEN + body_len;
    response_header(tx, req, payload_len as u32);
    write_status(tx, 0);
    deliver(sender_pid, tx, HDR_LEN + payload_len)
}
