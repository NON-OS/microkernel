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

use nonos_libc::mk_ipc_call_timeout;

use crate::wire::{build_request, read_i32, HDR_LEN};

pub fn call(
    port: u32,
    op: u16,
    request_id: u32,
    body: &[u8],
    rx: &mut [u8],
) -> Result<(i32, usize), &'static str> {
    call_within(port, op, request_id, body, rx, 0)
}

pub fn call_within(
    port: u32,
    op: u16,
    request_id: u32,
    body: &[u8],
    rx: &mut [u8],
    timeout_ms: u64,
) -> Result<(i32, usize), &'static str> {
    let tx = build_request(super::types::MAGIC, op, request_id, body);
    let rc = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        timeout_ms,
    );
    if rc <= 0 || (rc as usize) < HDR_LEN + 4 {
        return Err("vfs ipc failed");
    }
    Ok((read_i32(rx, HDR_LEN)?, rc as usize))
}
