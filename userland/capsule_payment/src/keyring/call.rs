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

//! One request to the keyring, found by name each time: a keyring that
//! restarted has a new port, and a stale one would answer nothing.

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::{mk_ipc_call, mk_service_lookup};
use nonos_payment_proto::EAGAIN;

const KEYRING: &[u8] = b"keyring";

/// The reply buffer and how many bytes of it arrived, once the status is 0.
fn exchange(op: u16, payload: &[u8], cap: usize) -> Result<(Vec<u8>, usize), i32> {
    let (mut port, mut pid) = (0u32, 0u32);
    if mk_service_lookup(KEYRING.as_ptr(), KEYRING.len(), &mut port, &mut pid) != 0 {
        return Err(EAGAIN);
    }
    let mut tx = Vec::with_capacity(8 + payload.len());
    tx.extend_from_slice(&0u32.to_le_bytes());
    tx.extend_from_slice(&op.to_le_bytes());
    tx.extend_from_slice(&0u16.to_le_bytes());
    tx.extend_from_slice(payload);
    let mut rx = vec![0u8; 8 + cap];
    let n = mk_ipc_call(port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len());
    if n < 8 {
        return Err(EAGAIN);
    }
    let status = i32::from_le_bytes([rx[4], rx[5], rx[6], rx[7]]);
    if status != 0 {
        return Err(status);
    }
    Ok((rx, n as usize))
}

/// The whole reply, header and exactly `body_len` bytes of body after it, or
/// the keyring's status. The body starts at 8; a caller holding a secret
/// wipes the whole buffer.
pub(crate) fn call(op: u16, payload: &[u8], body_len: usize) -> Result<Vec<u8>, i32> {
    let (rx, n) = exchange(op, payload, body_len)?;
    if n != 8 + body_len {
        return Err(EAGAIN);
    }
    Ok(rx)
}

/// The body alone, of any length up to `max`.
pub(crate) fn call_upto(op: u16, payload: &[u8], max: usize) -> Result<Vec<u8>, i32> {
    let (rx, n) = exchange(op, payload, max)?;
    rx.get(8..n).map(<[u8]>::to_vec).ok_or(EAGAIN)
}
