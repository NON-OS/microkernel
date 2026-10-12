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

//! The keyring's record operations as the Bluetooth capsule uses them:
//! OP_STORE, OP_RETRIEVE, OP_DELETE, and OP_LIST of the caller's own
//! records of one type (lead unit L33). A record is owned by the storing
//! capsule's service name, so a restarted capsule finds its own again.

use alloc::vec::Vec;

/// Key type 10: one bonded peer, a `BondRecord` (48 octets).
pub const KIND_BOND: u8 = 10;
/// Key type 11: this host's IRK, a `LocalIrk` (17 octets).
pub const KIND_LOCAL_IRK: u8 = 11;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeysError {
    /// The keyring did not answer, or answered with nothing usable.
    Unavailable,
    /// No room for another record of this owner.
    Full,
    /// No record with that id.
    NotFound,
    /// The record is not this owner's.
    Denied,
}

pub trait Keys {
    /// Keep `data` as a record of `kind`; the id it is kept under.
    fn store(&mut self, kind: u8, data: &[u8]) -> Result<u32, KeysError>;
    /// The record kept under `id`. It holds keys: the caller wipes it.
    fn retrieve(&mut self, id: u32) -> Result<Vec<u8>, KeysError>;
    fn delete(&mut self, id: u32) -> Result<(), KeysError>;
    /// The ids of this owner's records of `kind`.
    fn list(&mut self, kind: u8) -> Result<Vec<u32>, KeysError>;
}
