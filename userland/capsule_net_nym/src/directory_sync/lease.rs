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

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup, mk_uptime_ms, mk_yield};

const SERVICE: &[u8] = b"net.dhcp.client";
const MAGIC: u32 = 0x4E44_4843;
const VERSION: u16 = 1;
const HDR_LEN: usize = 20;
const BODY_LEN: usize = 22;
const BODY_MIN: usize = 18;
const OP_LEASE_STATUS: u16 = 3;
const STATE_BOUND: u8 = 3;
const REPLY_TIMEOUT_MS: u64 = 64;
const POLL_MS: i64 = 250;
const DEADLINE_MS: i64 = 30_000;
const E_NO_LEASE: u16 = 25;

/// Block until DHCP binds: the directory is dialled by address, not via DNS.
pub(super) fn wait_for_lease() -> Result<(), u16> {
    let deadline = mk_uptime_ms().saturating_add(DEADLINE_MS);
    loop {
        if online() {
            return Ok(());
        }
        let resume = mk_uptime_ms().saturating_add(POLL_MS);
        if resume > deadline {
            return Err(E_NO_LEASE);
        }
        while mk_uptime_ms() < resume {
            mk_yield();
        }
    }
}

fn online() -> bool {
    let mut port = 0u32;
    let mut pid = 0u32;
    let rc = mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port, &mut pid);
    if rc < 0 || port == 0 {
        return false;
    }
    let mut tx = [0u8; HDR_LEN];
    tx[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    tx[4..6].copy_from_slice(&VERSION.to_le_bytes());
    tx[6..8].copy_from_slice(&OP_LEASE_STATUS.to_le_bytes());
    let mut rx = [0u8; HDR_LEN + BODY_LEN];
    let n = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        REPLY_TIMEOUT_MS,
    );
    if n < (HDR_LEN + BODY_MIN) as i64 {
        return false;
    }
    let body_len = u32::from_le_bytes([rx[16], rx[17], rx[18], rx[19]]);
    let framed = rx[..8] == tx[..8] && rx[8..10] == [0, 0];
    framed && body_len >= BODY_MIN as u32 && rx[HDR_LEN] >= STATE_BOUND
}
