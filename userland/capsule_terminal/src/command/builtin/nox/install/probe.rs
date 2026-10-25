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

//! Whether the installer has finished a load, and how it ended. It serves
//! one request at a time, in the order they came, so once it answers a
//! question sent after the load, the load is over, and the answer is the
//! status it kept for it (its server/outcomes.rs): a refusal reaches the
//! terminal with its reason though the load's own reply came too late. A
//! busy installer answers only later, into a reply the terminal no longer
//! waits for, and the kernel drops it there.

use alloc::vec::Vec;

use nonos_app_skeleton::discover::lookup_service;
use nonos_libc::mk_ipc_call_timeout;

/// Hand-synced with `capsule_installer/src/protocol/types.rs`.
const OP_LOAD_OUTCOME: u16 = 9;
const SEQ: u32 = 1;
/// The installer answers at once when it is free.
const PROBE_TIMEOUT_MS: u64 = 20;
const HDR_LEN: usize = 8;

/// How the terminal's last load of `name` ended; None while the installer
/// is still too busy to say.
pub(super) fn probe(name: &[u8]) -> Option<i32> {
    let port = lookup_service(b"installer")?.port;
    // The body is a load's own: caps word, name, no argv.
    let mut tx = Vec::with_capacity(HDR_LEN + 9 + name.len());
    tx.extend_from_slice(&SEQ.to_le_bytes());
    tx.extend_from_slice(&OP_LOAD_OUTCOME.to_le_bytes());
    tx.extend_from_slice(&[0, 0]);
    tx.extend_from_slice(&0u64.to_le_bytes());
    tx.push(name.len() as u8);
    tx.extend_from_slice(name);
    let mut rx = [0u8; HDR_LEN];
    let rc = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        PROBE_TIMEOUT_MS,
    );
    (rc >= HDR_LEN as i64).then(|| i32::from_le_bytes([rx[4], rx[5], rx[6], rx[7]]))
}
