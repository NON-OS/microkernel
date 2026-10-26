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

//! Ask the installer to load a capsule-store app by name. Only the name and the
//! cap ceiling travel here; the installer reads the artifacts from the store
//! itself, so the request stays small however large the capsule is. A missing
//! installer, a refusal and a short reply all come back as Refused carrying the
//! installer's status, so the caller can say why the launch failed.
//!
//! The shell's frame loop is the thread that asks, so the answer is waited for
//! only briefly: a load the installer is still running when the wait ends is
//! InFlight, and the caller follows it on later turns (`state/launch.rs`).

use nonos_libc::mk_ipc_call_timeout;

use super::constants::{HDR_LEN, ISSUE_TIMEOUT_MS, MAX_NAME, OP_LOAD_BY_NAME};
use super::load_wire::{request, Issued, ERR_INVALID, ERR_NOT_READY, ERR_TIMEDOUT, PID_END};
use super::port::port;

pub fn issue_load(name: &[u8]) -> Issued {
    if name.is_empty() || name.len() > MAX_NAME {
        return Issued::Refused(ERR_INVALID);
    }
    let tx = request(OP_LOAD_BY_NAME, name);
    let mut rx = [0u8; 32];
    let Some(port) = port() else {
        return Issued::Refused(ERR_NOT_READY);
    };
    let rc = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        ISSUE_TIMEOUT_MS,
    );
    if rc == ERR_TIMEDOUT {
        return Issued::InFlight;
    }
    if rc <= 0 || (rc as usize) < HDR_LEN {
        return Issued::Refused(ERR_NOT_READY);
    }
    let status = i32::from_le_bytes([rx[4], rx[5], rx[6], rx[7]]);
    if status != 0 {
        return Issued::Refused(status);
    }
    if (rc as usize) < PID_END {
        return Issued::Refused(ERR_NOT_READY);
    }
    match u32::from_le_bytes([rx[8], rx[9], rx[10], rx[11]]) {
        0 => Issued::Refused(ERR_NOT_READY),
        pid => Issued::Loaded(pid),
    }
}
