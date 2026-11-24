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

//! What a machine vouches for when it vouches for a policy, and the prefix
//! every such domain shares.
//!
//! A local trailer signs the BLAKE3 of the bytes it is minted over, and the
//! kernel records that BLAKE3 among a guest family's measurements when it
//! verifies one, whatever the bytes are. A program and a policy are told
//! apart by what was signed: a policy is vouched for as `VOUCH_DOMAIN`
//! followed by its 256 bytes, and the machine mints nothing plain that
//! begins with `RESERVED` (capsule_linux install/record/vouch.rs). So the
//! measurement `vouched_hash` gives can come only from a policy's trailer;
//! a program, an index or any other blob the machine signed never makes it.

/// The prefix of every domain this machine signs under. Nothing minted
/// plain may begin with it.
pub const RESERVED: &[u8] = b"NONOS.linux.";

/// The domain a policy is vouched for under: its identity's own, so what
/// the kernel measures for a vouched policy is its `policy_hash`.
pub const VOUCH_DOMAIN: &[u8] = crate::identity::POLICY_DOMAIN;

/// True when `bytes` begin with a signing domain, so may not be minted
/// plain, nor taken as a program.
pub fn in_a_domain(bytes: &[u8]) -> bool {
    bytes.starts_with(RESERVED)
}

/// The BLAKE3 of what a machine vouches for when it vouches for `policy`:
/// the measurement the kernel records when it verifies that trailer, and,
/// for a policy's 256 bytes, its `policy_hash`.
pub fn vouched_hash(policy: &[u8]) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(VOUCH_DOMAIN);
    h.update(policy);
    *h.finalize().as_bytes()
}
