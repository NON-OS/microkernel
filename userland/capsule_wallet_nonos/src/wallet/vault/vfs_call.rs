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

//! Sending one request to vfs_pool and reading its status back.

use alloc::vec::Vec;

use nonos_libc::mk_ipc_call_timeout;

use super::super::answer::Answer;
use super::{HDR_LEN, MAGIC, VERSION, VFS_PORT};

/// Longer than the shell's 300ms: this runs when the user saves or restores a
/// wallet, not on a paint path, and the store may be busy staging packages.
const TIMEOUT_MS: u64 = 1000;

pub(in crate::wallet::vault) fn call(op: u16, body: &[u8], rx: &mut [u8]) -> Answer {
    let mut tx = Vec::with_capacity(HDR_LEN + body.len());
    tx.extend_from_slice(&MAGIC.to_le_bytes());
    tx.extend_from_slice(&VERSION.to_le_bytes());
    tx.extend_from_slice(&op.to_le_bytes());
    tx.extend_from_slice(&0u16.to_le_bytes());
    tx.extend_from_slice(&0u16.to_le_bytes());
    tx.extend_from_slice(&1u32.to_le_bytes());
    tx.extend_from_slice(&(body.len() as u32).to_le_bytes());
    tx.extend_from_slice(body);
    let rc = mk_ipc_call_timeout(
        VFS_PORT as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        TIMEOUT_MS,
    );
    if rc <= 0 || (rc as usize) < HDR_LEN + 4 {
        return Answer::Silent;
    }
    let status =
        i32::from_le_bytes([rx[HDR_LEN], rx[HDR_LEN + 1], rx[HDR_LEN + 2], rx[HDR_LEN + 3]]);
    if status != 0 {
        return Answer::Refused(status);
    }
    Answer::Ok(rc as usize)
}
