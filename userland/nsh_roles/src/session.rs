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

//! What an admitted login and an admitted escalation hand the server: the
//! capabilities the session capsule holds, the time it must end by, and the
//! grant's hash for the record. They own their bytes, so they outlive the
//! frame the grant came in.

use alloc::vec::Vec;
use nonos_hash::sha512;
use nsh_wire::keys::DIGEST_LEN;
use nsh_wire::{Grant, GRANT_ID_LEN};

use crate::CapSet;

/// `H(grant)`: SHA-512 over the bytes the issuer signed and the issuer's
/// signature. The approvals are left out; the issuer does not sign them.
pub fn grant_hash(grant: &Grant<'_>) -> [u8; DIGEST_LEN] {
    sha512(grant.issued)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Login {
    pub grant_id: [u8; GRANT_ID_LEN],
    pub fleet_id: [u8; 16],
    pub subject: [u8; DIGEST_LEN],
    pub user: Vec<u8>,
    pub role: Vec<u8>,
    /// The capabilities the grant names, all within the role's ceiling.
    pub caps: CapSet,
    /// The grant's end, or the role's longest session, whichever is first.
    pub ends_ms: u64,
    pub hash: [u8; DIGEST_LEN],
    pub record_digest: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Escalation {
    pub grant_id: [u8; GRANT_ID_LEN],
    /// The capabilities of the second session capsule.
    pub caps: CapSet,
    /// `min(escalation not_after, base session end)`: the kernel's hard end.
    pub ends_ms: u64,
    pub hash: [u8; DIGEST_LEN],
}
