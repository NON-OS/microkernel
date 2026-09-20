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

//! One call to the market service, framed the way it frames replies.

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::mk_ipc_call_timeout;

const MAGIC: u32 = 0x4E4D_4B54;
const VERSION: u16 = 1;
pub const HDR_LEN: usize = 20;
const STATUS_LEN: usize = 4;

/// A catalogue reply carries every listing, so this is sized for the
/// catalogue rather than for one entry.
const RX_CAP: usize = 96 << 10;

/// Long enough for the capsule to walk its index, short enough that a
/// service that has stopped answering does not freeze a repaint.
const TIMEOUT_MS: u64 = 1500;

pub fn call(port: u32, op: u16, request_id: u32, body: &[u8]) -> Option<Vec<u8>> {
    if port == 0 {
        return None;
    }
    let mut tx = Vec::with_capacity(HDR_LEN + body.len());
    tx.extend_from_slice(&MAGIC.to_le_bytes());
    tx.extend_from_slice(&VERSION.to_le_bytes());
    tx.extend_from_slice(&op.to_le_bytes());
    tx.extend_from_slice(&0u16.to_le_bytes());
    tx.extend_from_slice(&0u16.to_le_bytes());
    tx.extend_from_slice(&request_id.to_le_bytes());
    tx.extend_from_slice(&(body.len() as u32).to_le_bytes());
    tx.extend_from_slice(body);

    let mut rx = vec![0u8; RX_CAP];
    let rc =
        mk_ipc_call_timeout(port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len(), TIMEOUT_MS);
    let got = usize::try_from(rc).ok()?;
    if got < HDR_LEN + STATUS_LEN {
        return None;
    }
    let status = i32::from_le_bytes(rx.get(HDR_LEN..HDR_LEN + STATUS_LEN)?.try_into().ok()?);
    if status != 0 {
        return None;
    }
    /*
     * The status word is part of the body on this protocol, and every reader
     * here wants what follows it.
     */
    Some(rx.get(HDR_LEN + STATUS_LEN..got)?.to_vec())
}
