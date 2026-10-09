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

use alloc::{vec, vec::Vec};

use nonos_libc::mk_ipc_call_timeout;

use super::constants::HDR_LEN;

// The keyring answers most ops in microseconds; HD generation and recovery
// run PBKDF2 with 2048 rounds plus derivation, still well under a second.
// The bound exists so a wedged keyring surfaces as an error on the UI
// thread instead of freezing the wallet.
const KEYRING_TIMEOUT_MS: u64 = 4000;

pub fn keyring_call(port: u32, op: u16, payload: &[u8], rx_len: usize) -> Result<Vec<u8>, i32> {
    let mut tx = Vec::with_capacity(HDR_LEN + payload.len());
    tx.extend_from_slice(&1u32.to_le_bytes());
    tx.extend_from_slice(&op.to_le_bytes());
    tx.extend_from_slice(&0u16.to_le_bytes());
    tx.extend_from_slice(payload);
    let mut rx = vec![0u8; HDR_LEN + rx_len];
    let rc = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        KEYRING_TIMEOUT_MS,
    );
    /* The request may carry a secret (an imported key, recovery words): no
     * copy of it stays in the heap. Nor does an answer that is not handed
     * on. */
    wipe(&mut tx);
    if rc < HDR_LEN as i64 {
        wipe(&mut rx);
        return Err(-11);
    }
    rx.truncate(rc as usize);
    let status = i32::from_le_bytes([rx[4], rx[5], rx[6], rx[7]]);
    if status != 0 {
        wipe(&mut rx);
        return Err(status);
    }
    Ok(rx)
}

/// Zero a buffer so the write is not optimised away.
pub fn wipe(buf: &mut [u8]) {
    for b in buf.iter_mut() {
        // SAFETY: volatile write to a byte this function holds a borrow of.
        unsafe { core::ptr::write_volatile(b, 0) };
    }
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}
