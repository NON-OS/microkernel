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

//! The router's SESSION_LOCK (0x0008): lock, with the rectangles where the
//! pointer is drawn while locked; let go; or ask. The answer is the state
//! after it.

use nonos_libc::mk_ipc_call_timeout;

use super::wire::{HDR_LEN, MAGIC, OP_SESSION_LOCK, REPLY_TIMEOUT_MS, STATUS_LEN, VERSION};
use crate::protocol::{read_i32, read_u32};

const MAX_SHOWN: usize = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LockWord {
    Release = 0,
    Lock = 1,
    Ask = 2,
}

/// The router's lock state after `word`, or Err when it did not answer.
pub fn session_lock(
    port: u32,
    request_id: u32,
    word: LockWord,
    shown: &[(u32, u32, u32, u32)],
) -> Result<bool, &'static str> {
    let n = shown.len().min(MAX_SHOWN);
    let body_len = 4 + 16 * n;
    let mut tx = [0u8; HDR_LEN + 4 + 16 * MAX_SHOWN];
    tx[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    tx[4..6].copy_from_slice(&VERSION.to_le_bytes());
    tx[6..8].copy_from_slice(&OP_SESSION_LOCK.to_le_bytes());
    tx[12..16].copy_from_slice(&request_id.to_le_bytes());
    tx[16..20].copy_from_slice(&(body_len as u32).to_le_bytes());
    tx[20..24].copy_from_slice(&(word as u32).to_le_bytes());
    for (i, r) in shown.iter().take(n).enumerate() {
        let at = HDR_LEN + 4 + i * 16;
        for (j, v) in [r.0, r.1, r.2, r.3].into_iter().enumerate() {
            tx[at + j * 4..at + j * 4 + 4].copy_from_slice(&v.to_le_bytes());
        }
    }
    let mut rx = [0u8; HDR_LEN + STATUS_LEN + 4];
    let rc = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        HDR_LEN + body_len,
        rx.as_mut_ptr(),
        rx.len(),
        REPLY_TIMEOUT_MS,
    );
    if rc < rx.len() as i64 {
        return Err("input_router call failed");
    }
    match (read_i32(&rx, HDR_LEN), read_u32(&rx, HDR_LEN + STATUS_LEN)) {
        (Some(0), Some(locked)) => Ok(locked == 1),
        _ => Err("input_router rejected request"),
    }
}
