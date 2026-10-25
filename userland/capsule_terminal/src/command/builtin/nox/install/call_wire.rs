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

//! One load-by-name request to the installer, and its reply, on the wire.

use nonos_app_skeleton::discover::lookup_service;
use nonos_libc::mk_ipc_call_timeout;

use super::ETIMEDOUT;

#[path = "call_pack.rs"]
mod call_pack;

use call_pack::pack;

const EAGAIN: i32 = -11;

pub(super) fn call_within(name: &[u8], args: &[u8], timeout_ms: u64) -> Result<u32, i32> {
    let port = lookup_service(b"installer").map(|p| p.port).ok_or(EAGAIN)?;
    let payload = pack(name, args);
    let mut rx = [0u8; 32];
    let rc = mk_ipc_call_timeout(
        port as u64,
        payload.as_ptr(),
        payload.len(),
        rx.as_mut_ptr(),
        rx.len(),
        timeout_ms,
    );
    if rc == ETIMEDOUT as i64 {
        return Err(ETIMEDOUT);
    }
    if rc < 8 {
        return Err(EAGAIN);
    }
    let status = i32::from_le_bytes([rx[4], rx[5], rx[6], rx[7]]);
    if status != 0 {
        return Err(status);
    }
    if rc < 12 {
        return Err(EAGAIN);
    }
    Ok(u32::from_le_bytes([rx[8], rx[9], rx[10], rx[11]]))
}
