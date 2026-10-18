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

//! The two hashes a policy is known by.
//!
//! The policy hash is what a release signs and a container carries; the app
//! id is what capsule_linux notes in the kernel's seal, so a proof names the
//! program and its box together and neither can be swapped under the other.

use crate::layout::LEN;

const POLICY_DOMAIN: &[u8] = b"NONOS.linux.policy.v1";
const APP_DOMAIN: &[u8] = b"NONOS.linux.app.v1";

/// BLAKE3("NONOS.linux.policy.v1" | the 256 bytes).
pub fn policy_hash(bytes: &[u8; LEN]) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(POLICY_DOMAIN);
    h.update(bytes);
    *h.finalize().as_bytes()
}

/// BLAKE3("NONOS.linux.app.v1" | program_blake3 | policy_hash).
pub fn app_id(program_blake3: &[u8; 32], policy_hash: &[u8; 32]) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(APP_DOMAIN);
    h.update(program_blake3);
    h.update(policy_hash);
    *h.finalize().as_bytes()
}
