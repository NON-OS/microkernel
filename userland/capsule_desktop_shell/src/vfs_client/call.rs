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

//! Send one request and hand back the reply length only when the server
//! answered with a success status. Every short or failed reply becomes None
//! (or, through `call_status`, the server's errno, or `NO_REPLY` when nothing
//! answered) so callers never read a half-formed buffer.

use alloc::vec::Vec;

use nonos_libc::mk_ipc_call_timeout;

use super::constants::{budget, HDR_LEN, VFS_PORT};
use super::frame::build;
use crate::state::says::NO_REPLY;

pub(super) fn call(op: u16, body: &[u8], rx: &mut [u8]) -> Option<usize> {
    call_status(op, body, rx).ok()
}

/// As `call`, with the reason a request did not succeed: the server's errno,
/// or `NO_REPLY` when no whole reply came back.
pub(super) fn call_status(op: u16, body: &[u8], rx: &mut [u8]) -> Result<usize, i32> {
    let tx: Vec<u8> = build(op, body);
    let rc = mk_ipc_call_timeout(
        VFS_PORT as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        budget(op),
    );
    if rc <= 0 || (rc as usize) < HDR_LEN + 4 {
        return Err(NO_REPLY);
    }
    let status =
        i32::from_le_bytes([rx[HDR_LEN], rx[HDR_LEN + 1], rx[HDR_LEN + 2], rx[HDR_LEN + 3]]);
    if status != 0 {
        return Err(status);
    }
    Ok(rc as usize)
}
