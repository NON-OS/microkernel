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

//! Direct calls to the crypto pool, and finding it.

use crate::serve::input;

/// The port the stand-in pool answers on.
pub const POOL_PORT: u32 = 4102;
const POOL_PID: u32 = 5;
const ENOENT: i64 = -2;

pub fn mk_service_lookup(name: *const u8, len: usize, port: *mut u32, pid: *mut u32) -> i64 {
    crate::count::note(|c| c.lookups += 1);
    if input(name, len) != b"crypto_pool" {
        return ENOENT;
    }
    unsafe {
        *port = POOL_PORT;
        *pid = POOL_PID;
    }
    0
}

pub fn mk_ipc_call(
    endpoint: u64,
    req: *const u8,
    req_len: usize,
    resp: *mut u8,
    cap: usize,
) -> i64 {
    if endpoint != POOL_PORT as u64 {
        return ENOENT;
    }
    let message = input(req, req_len);
    let op = message.get(6..8).map_or(0, |b| u16::from_le_bytes([b[0], b[1]]));
    crate::count::note(|c| match op {
        18 => c.p256 += 1,
        19 => c.p384 += 1,
        20 => c.sha384 += 1,
        21 => c.rsa += 1,
        _ => c.other_ipc += 1,
    });
    let reply = crate::serve::answer(message);
    let n = reply.len().min(cap);
    unsafe { core::ptr::copy_nonoverlapping(reply.as_ptr(), resp, n) };
    n as i64
}
