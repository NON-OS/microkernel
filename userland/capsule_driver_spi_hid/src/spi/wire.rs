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

//! driver.spi0's NSPI wire from the holder's side: the host wire's 20-byte
//! header (magic, version, op, request id, payload length), then the body;
//! a reply's payload starts with its status.

use alloc::vec;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

use nonos_libc::mk_ipc_call_timeout;

pub const MAGIC: u32 = 0x4E53_5049;
pub const HDR_LEN: usize = 20;
const VERSION: u16 = 1;
pub const OP_OPEN: u16 = 2;
pub const OP_TRANSFER: u16 = 4;
pub const OP_DOORBELL: u16 = 5;
/// spi0's most bytes one transfer moves, both ways at once.
pub const TRANSFER_MAX: usize = 4000;
/// The wire's I/O error: a call that brought no reply, or not one.
pub const E_IO: i32 = -5;
/// A whole transfer is bounded at 100 ms in spi0; the call waits longer.
const CALL_MS: u64 = 500;

static SEQ: AtomicU32 = AtomicU32::new(1);

/// One call to spi0 at `port`: the reply's data, or its status.
pub fn call(port: u32, op: u16, body: &[u8]) -> Result<Vec<u8>, i32> {
    let mut req = vec![0u8; HDR_LEN + body.len()];
    req[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    req[4..6].copy_from_slice(&VERSION.to_le_bytes());
    req[6..8].copy_from_slice(&op.to_le_bytes());
    req[12..16].copy_from_slice(&SEQ.fetch_add(1, Ordering::Relaxed).to_le_bytes());
    req[16..20].copy_from_slice(&(body.len() as u32).to_le_bytes());
    req[HDR_LEN..].copy_from_slice(body);
    let mut resp = vec![0u8; HDR_LEN + 4 + 16 + TRANSFER_MAX];
    let (rp, rl) = (resp.as_mut_ptr(), resp.len());
    let n = mk_ipc_call_timeout(port as u64, req.as_ptr(), req.len(), rp, rl, CALL_MS);
    let n = usize::try_from(n).map_err(|_| E_IO)?;
    reply(&resp[..n.min(rl)])
}

/// A reply's data when its status is 0; a reply shorter than its own
/// payload length says is no reply.
pub fn reply(r: &[u8]) -> Result<Vec<u8>, i32> {
    let word = |i: usize| r.get(i..i + 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]));
    if word(0) != Some(MAGIC) {
        return Err(E_IO);
    }
    let payload = word(16).ok_or(E_IO)? as usize;
    let status = word(20).ok_or(E_IO)? as i32;
    let data = r.get(HDR_LEN + 4..HDR_LEN.saturating_add(payload)).ok_or(E_IO)?;
    match status {
        0 => Ok(data.to_vec()),
        s => Err(s),
    }
}
