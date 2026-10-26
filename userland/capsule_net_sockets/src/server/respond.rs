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

use nonos_libc::ipc::mk_ipc_reply_to;
use nonos_libc::mk_ipc_reply;

use crate::protocol::MAGIC;

const HDR_LEN: usize = 20;

/// Answer the request `pid` sent last, as it is being handled.
pub fn respond(pid: u32, op: u16, errno: u16, rid: u32, payload_len: u32, tx: &mut [u8]) -> i64 {
    let n = frame(op, errno, rid, payload_len, tx);
    mk_ipc_reply(pid, tx.as_ptr(), n)
}

/// Answer a request held past its turn by the kernel's id for it, `ipc`
/// (request.rs): that call and no other. A call that gave up meanwhile is
/// ETIMEDOUT and gets nothing.
pub fn respond_to(ipc: u64, op: u16, errno: u16, rid: u32, tx: &mut [u8]) -> i64 {
    let n = frame(op, errno, rid, 0, tx);
    mk_ipc_reply_to(ipc, tx.as_ptr(), n)
}

fn frame(op: u16, errno: u16, rid: u32, payload_len: u32, tx: &mut [u8]) -> usize {
    tx[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    tx[4..6].copy_from_slice(&1u16.to_le_bytes());
    tx[6..8].copy_from_slice(&op.to_le_bytes());
    tx[8..10].copy_from_slice(&errno.to_le_bytes());
    tx[10..12].fill(0);
    tx[12..16].copy_from_slice(&rid.to_le_bytes());
    tx[16..20].copy_from_slice(&payload_len.to_le_bytes());
    HDR_LEN + payload_len as usize
}
