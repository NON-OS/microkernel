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

//! Grabbing input kinds for the shell, and letting them go.

use nonos_libc::mk_ipc_call_timeout;
use crate::protocol::read_i32;
use super::wire::{BODY_LEN, HDR_LEN, MAGIC, OP_GRAB_RELEASE, OP_GRAB_REQUEST, REPLY_TIMEOUT_MS, STATUS_LEN, VERSION};

/// Grab the given input kinds so they route to us regardless of window focus.
/// Used for the modal desktop rename; released the moment it ends.
pub fn grab(port: u32, request_id: u32, kind_mask: u32) -> Result<(), &'static str> {
    grab_call(port, OP_GRAB_REQUEST, request_id, Some(kind_mask))
}

/// Release any grab this shell holds.
pub fn release_grab(port: u32, request_id: u32) -> Result<(), &'static str> {
    grab_call(port, OP_GRAB_RELEASE, request_id, None)
}

fn grab_call(port: u32, op: u16, request_id: u32, body: Option<u32>) -> Result<(), &'static str> {
    let body_len = if body.is_some() { BODY_LEN } else { 0 };
    let mut tx = [0u8; HDR_LEN + BODY_LEN];
    tx[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    tx[4..6].copy_from_slice(&VERSION.to_le_bytes());
    tx[6..8].copy_from_slice(&op.to_le_bytes());
    tx[12..16].copy_from_slice(&request_id.to_le_bytes());
    tx[16..20].copy_from_slice(&(body_len as u32).to_le_bytes());
    if let Some(v) = body {
        tx[20..24].copy_from_slice(&v.to_le_bytes());
    }
    let mut rx = [0u8; HDR_LEN + STATUS_LEN];
    let rc = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        HDR_LEN + body_len,
        rx.as_mut_ptr(),
        rx.len(),
        REPLY_TIMEOUT_MS,
    );
    if rc < (HDR_LEN + STATUS_LEN) as i64 {
        return Err("input_router call failed");
    }
    match read_i32(&rx, HDR_LEN) {
        Some(0) => Ok(()),
        _ => Err("input_router rejected request"),
    }
}
