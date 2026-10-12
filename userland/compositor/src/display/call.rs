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

//! One NVGP v1 request to a display driver on one of its outputs, and its
//! reply held to the request (same op, output and id) by `nonos_gfx_wire`,
//! so the compositor and every driver read the same bytes the same way.

use core::sync::atomic::{AtomicU32, Ordering};

use nonos_gfx_wire::errno::{EINVAL, ETIMEDOUT};
use nonos_gfx_wire::{Errno, Header, Reply, HDR_LEN, MAX_REPLY_BODY, MAX_REQUEST};
use nonos_libc::mk_ipc_call_timeout;

/// A present that misses is one late frame, not a stalled desktop.
pub const PRESENT_MS: u64 = 100;
/// Setup and polls: a driver answering from its own loop, never the device.
pub const SETUP_MS: u64 = 250;

static NEXT_ID: AtomicU32 = AtomicU32::new(1);

/// The status and the body's length in `body`. No reply in time is
/// ETIMEDOUT, a reply that does not answer this request EINVAL.
pub fn call(
    (port, output): (u32, u16),
    op: u16,
    payload: &[u8],
    body: &mut [u8],
    ms: u64,
) -> Result<usize, Errno> {
    let request_id = NEXT_ID.fetch_add(1, Ordering::Relaxed).max(1);
    let payload_len = payload.len() as u32;
    let header = Header { op, flags: 0, output, request_id, payload_len };
    let mut tx = [0u8; MAX_REQUEST];
    let end = HDR_LEN + payload.len();
    let frame = tx.get_mut(..end).ok_or(EINVAL)?;
    header.encode(frame)?;
    frame[HDR_LEN..].copy_from_slice(payload);
    let mut rx = [0u8; HDR_LEN + 4 + MAX_REPLY_BODY];
    let rc = mk_ipc_call_timeout(port as u64, tx.as_ptr(), end, rx.as_mut_ptr(), rx.len(), ms);
    if rc <= 0 {
        return Err(ETIMEDOUT);
    }
    let reply = Reply::decode(&rx[..rc as usize], &header)?;
    if reply.status != 0 {
        return Err(reply.status);
    }
    let n = reply.body.len().min(body.len());
    body[..n].copy_from_slice(&reply.body[..n]);
    Ok(n)
}
