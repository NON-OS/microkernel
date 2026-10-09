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

//! One request to driver.xhci0 and its reply.

use core::sync::atomic::{AtomicU32, Ordering};

use nonos_libc::mk_ipc_call_timeout;

use super::wire::{BULK_MAX, DATA_AT, E_IO, HDR_LEN, MAGIC, VERSION};

/// Longer than xHCI's own wait for a transfer (5 s) plus its endpoint
/// recovery, so the controller, not this call, says why a transfer failed.
const CALL_TIMEOUT_MS: u64 = 12_000;

static SEQ: AtomicU32 = AtomicU32::new(1);

/// Send `op` with `body`, 16 + BULK_MAX bytes at most; the reply lands in
/// `resp`. Returns the reply's data length after the status word, or the
/// negative status, or `E_IO` when the call or its framing failed.
pub fn call(port: u32, op: u16, body: &[u8], resp: &mut [u8]) -> Result<usize, i32> {
    let mut req = [0u8; HDR_LEN + 16 + BULK_MAX];
    let total = HDR_LEN + body.len();
    if total > req.len() || resp.len() < DATA_AT {
        return Err(E_IO);
    }
    let rid = SEQ.fetch_add(1, Ordering::Relaxed).max(1);
    req[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    req[4..6].copy_from_slice(&VERSION.to_le_bytes());
    req[6..8].copy_from_slice(&op.to_le_bytes());
    req[12..16].copy_from_slice(&rid.to_le_bytes());
    req[16..20].copy_from_slice(&(body.len() as u32).to_le_bytes());
    req[HDR_LEN..total].copy_from_slice(body);
    let (rp, rl) = (resp.as_mut_ptr(), resp.len());
    let n = mk_ipc_call_timeout(port as u64, req.as_ptr(), total, rp, rl, CALL_TIMEOUT_MS);
    if n < DATA_AT as i64 {
        return Err(E_IO);
    }
    let word = |at: usize| u32::from_le_bytes([resp[at], resp[at + 1], resp[at + 2], resp[at + 3]]);
    let plen = word(16) as usize;
    if word(0) != MAGIC || word(12) != rid || plen < 4 || HDR_LEN + plen > resp.len() {
        return Err(E_IO);
    }
    match word(HDR_LEN) as i32 {
        0 => Ok(plen - 4),
        status => Err(status),
    }
}
