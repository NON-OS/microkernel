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

//! Why a role table or a revocation list was refused. Each is refused
//! whole: a host never runs on part of a table.

use nsh_wire::WireError;

/// Why a list of names is not a set this host can hold.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapError {
    /// A name the kernel does not have.
    Unknown,
    /// A name no role may ever hold.
    Forbidden,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyError {
    /// The bytes do not parse: a length, magic, version, reserved byte or
    /// name shape is wrong.
    Wire(WireError),
    /// A number is outside its range: a count, an age, a session length,
    /// an approval count, or a revoked id of all zeros.
    Range,
    /// Keys, roles or revoked ids are not in strictly ascending order.
    Order,
    /// A key is both a role authority and an approver.
    Overlap,
    /// A ceiling names a capability the kernel does not have.
    CapUnknown,
    /// A ceiling names a capability no role may ever hold.
    CapForbidden,
    /// A ceiling that grants nothing.
    EmptyCeiling,
    /// The signer is not the key the host pins, or not a role authority.
    Signer,
    /// A signature does not verify.
    Signature,
    /// The revocation list is for another fleet.
    Fleet,
    /// Older than the newest this host has installed: a lower table serial,
    /// or a revocation list issued earlier.
    Replayed,
}

impl From<WireError> for PolicyError {
    fn from(e: WireError) -> Self {
        PolicyError::Wire(e)
    }
}

impl From<CapError> for PolicyError {
    fn from(e: CapError) -> Self {
        match e {
            CapError::Unknown => PolicyError::CapUnknown,
            CapError::Forbidden => PolicyError::CapForbidden,
        }
    }
}
