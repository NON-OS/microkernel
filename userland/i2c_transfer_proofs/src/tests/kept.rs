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

//! A client's OP_TRANSFER answered on a reply buffer the size the server
//! keeps (server/runner.rs), and a look in that buffer afterwards.

use nonos_libc::take_reply;

use super::fixture::Bench;
use crate::protocol::{Request, HDR_LEN, IPC_PAYLOAD_MAX, OP_TRANSFER};
use crate::server::handlers::transfer;
use crate::transaction::FLAG_RESTART_ON_READ;

/// The input register, where a pad's reports are read.
pub const INPUT: [u8; 2] = [0x03, 0x00];

/// A transfer body: write `write` to `addr`, then read `read_len` bytes
/// after a repeated START.
pub fn body(addr: u8, write: &[u8], read_len: usize) -> Vec<u8> {
    let mut b = vec![addr, 0];
    b.extend((write.len() as u16).to_le_bytes());
    b.extend((read_len as u16).to_le_bytes());
    b.extend(FLAG_RESTART_ON_READ.to_le_bytes());
    [b, write.to_vec()].concat()
}

/// A reply buffer as large as the server's.
pub fn reply_buffer() -> Vec<u8> {
    vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX]
}

/// The reply sent for an OP_TRANSFER of `body`, and the reply buffer after
/// the handler returned.
pub fn answered(b: &Bench, body: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let mut out = reply_buffer();
    transfer::handle(&b.driver, 42, &Request { op: OP_TRANSFER, request_id: 7 }, body, &mut out);
    (take_reply().expect("the handler replied").1, out)
}

/// Every byte is zero.
pub fn wiped(buf: &[u8]) -> bool {
    buf.iter().all(|&b| b == 0)
}
