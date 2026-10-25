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

//! The load request's body and what its answers mean.

use alloc::vec::Vec;

use super::constants::{HDR_LEN, REQUESTED_CAPS, SEQ};

/// Reply is seq(4) | status(4) | pid(4); nothing past the pid is read.
pub(super) const PID_END: usize = HDR_LEN + 4;

/// Local stand-in when the installer cannot be reached or replies short.
pub(super) const ERR_NOT_READY: i32 = -11;

/// Local stand-in for a name the request cannot even carry.
pub(super) const ERR_INVALID: i32 = -22;

/// The kernel's answer when the wait ran out with the request delivered.
pub(super) const ERR_TIMEDOUT: i64 = -110;

/// How a load request went within the wait it was given.
pub enum Issued {
    Loaded(u32),
    Refused(i32),
    /// Delivered, and the installer is still at it.
    InFlight,
}

/// Body after the header is requested_caps(8) | name_len(1) | name | args, with
/// no args: the Launchpad launches an app plainly, as a dock click does.
/// OP_LOAD_OUTCOME takes the same body, to ask how that load ended.
pub(super) fn request(op: u16, name: &[u8]) -> Vec<u8> {
    let mut tx = Vec::with_capacity(HDR_LEN + 9 + name.len());
    tx.extend_from_slice(&SEQ.to_le_bytes());
    tx.extend_from_slice(&op.to_le_bytes());
    tx.extend_from_slice(&[0u8, 0u8]);
    tx.extend_from_slice(&REQUESTED_CAPS.to_le_bytes());
    tx.push(name.len() as u8);
    tx.extend_from_slice(name);
    tx
}
