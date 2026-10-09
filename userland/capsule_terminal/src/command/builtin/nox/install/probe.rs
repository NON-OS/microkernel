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

//! Whether the installer has caught up. It serves one request at a time, in
//! the order they came, so once it answers a probe every request sent before
//! that probe, a load among them, is done. A busy installer answers the probe
//! only later, into a reply the terminal no longer waits for, and the kernel
//! drops it there.

use nonos_app_skeleton::discover::lookup_service;
use nonos_libc::mk_ipc_call_timeout;

/// Hand-synced with `capsule_installer/src/protocol/types.rs`.
const OP_HEALTHCHECK: u16 = 1;
const SEQ: u32 = 1;
/// The installer answers a health probe at once when it is free.
const PROBE_TIMEOUT_MS: u64 = 20;
const HDR_LEN: usize = 8;

pub(super) fn probe() -> bool {
    let Some(port) = lookup_service(b"installer").map(|p| p.port) else {
        return false;
    };
    let mut tx = [0u8; HDR_LEN];
    tx[0..4].copy_from_slice(&SEQ.to_le_bytes());
    tx[4..6].copy_from_slice(&OP_HEALTHCHECK.to_le_bytes());
    let mut rx = [0u8; HDR_LEN];
    let rc = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        PROBE_TIMEOUT_MS,
    );
    rc >= HDR_LEN as i64
}
