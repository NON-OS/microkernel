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

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup};

use crate::term::state::State;

const SERVICE: &[u8] = b"net.dhcp.client";
const MAGIC: u32 = 0x4E44_4843;
const HDR_LEN: usize = 20;
const BODY_LEN: usize = 18;
const OP_LEASE_STATUS: u16 = 3;
const TIMEOUT_MS: u64 = 64;

#[path = "ifconfig_line.rs"]
mod ifconfig_line;

use ifconfig_line::lease_line;

pub fn run(state: &mut State) -> bool {
    let mut port = 0u32;
    let mut pid = 0u32;
    let rc = mk_service_lookup(
        SERVICE.as_ptr(),
        SERVICE.len(),
        &mut port as *mut u32,
        &mut pid as *mut u32,
    );
    if rc < 0 || port == 0 {
        state.scrollback.push_line(b"net0: down");
        return true;
    }
    let mut tx = [0u8; HDR_LEN];
    tx[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    tx[4..6].copy_from_slice(&1u16.to_le_bytes());
    tx[6..8].copy_from_slice(&OP_LEASE_STATUS.to_le_bytes());
    let mut rx = [0u8; HDR_LEN + BODY_LEN];
    let n = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        TIMEOUT_MS,
    );
    if n < (HDR_LEN + BODY_LEN) as i64 || rx[HDR_LEN] < 3 {
        state.scrollback.push_line(b"net0: down");
        return true;
    }
    let line = lease_line(&rx);
    state.scrollback.push_line(&line);
    true
}
