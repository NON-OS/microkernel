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

use alloc::vec::Vec;

use nonos_libc::mk_ipc_call_timeout;

use super::constants::{HDR_LEN, ISSUE_TIMEOUT_MS, MAX_NAME, OP_LOAD_BY_NAME, REQUESTED_CAPS, SEQ};
use super::port::port;

/// Reply is seq(4) | status(4) | pid(4); nothing past the pid is read.
const PID_END: usize = HDR_LEN + 4;

/// Local stand-in when the installer cannot be reached or replies short.
const ERR_NOT_READY: i32 = -11;
/// Local stand-in for a name the request cannot even carry.
const ERR_INVALID: i32 = -22;
/// The kernel's answer when the wait ran out with the request delivered.
const ERR_TIMEDOUT: i64 = -110;

/// How a load request went within the wait it was given.
pub enum Issued {
    Loaded(u32),
    Refused(i32),
    /// Delivered, and the installer is still at it.
    InFlight,
}

pub fn issue_load(name: &[u8]) -> Issued {
    if name.is_empty() || name.len() > MAX_NAME {
        return Issued::Refused(ERR_INVALID);
    }
    let tx = request(name);
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

/// Body after the header is requested_caps(8) | name_len(1) | name | args, with
/// no args: the Launchpad launches an app plainly, as a dock click does.
fn request(name: &[u8]) -> Vec<u8> {
    let mut tx = Vec::with_capacity(HDR_LEN + 9 + name.len());
    tx.extend_from_slice(&SEQ.to_le_bytes());
    tx.extend_from_slice(&OP_LOAD_BY_NAME.to_le_bytes());
    tx.extend_from_slice(&[0u8, 0u8]);
    tx.extend_from_slice(&REQUESTED_CAPS.to_le_bytes());
    tx.push(name.len() as u8);
    tx.extend_from_slice(name);
    tx
}
