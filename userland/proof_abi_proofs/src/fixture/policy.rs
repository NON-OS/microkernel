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

//! A LinuxAppPolicy as the plan lays it out, and a reader of the two fields
//! the verify order takes. The policy's real decoder is C's
//! nonos_linux_policy; this one reads only what the order needs.

use nonos_proof_abi::{PolicyView, SET_BYTES};

pub const PROGRAM: [u8; 32] = [0xA7; 32];

/// A policy for PROGRAM whose set lets `allowed` succeed.
pub fn policy(allowed: &[usize]) -> [u8; 256] {
    let mut p = [0u8; 256];
    p[..8].copy_from_slice(b"NLXPOL01");
    p[8..10].copy_from_slice(&1u16.to_le_bytes());
    p[16..48].copy_from_slice(&PROGRAM);
    p[48..56].copy_from_slice(&0b10_0001u64.to_le_bytes());
    for &b in allowed {
        p[128 + b / 8] |= 1 << (b % 8);
    }
    p
}

pub fn decode_policy(b: &[u8]) -> Option<PolicyView> {
    if b.len() != 256 || b[..8] != *b"NLXPOL01" || b[8..10] != 1u16.to_le_bytes() {
        return None;
    }
    let mut set = [0u8; SET_BYTES];
    set.copy_from_slice(&b[128..128 + SET_BYTES]);
    Some(PolicyView { program_blake3: b[16..48].try_into().ok()?, syscall_set: set })
}

/// the plan's app_id, written out here apart from the crate's own, so the two
/// are held to each other: BLAKE3("NONOS.linux.app.v1" | program | policy_hash).
pub fn app_id(policy: &[u8; 256]) -> [u8; 32] {
    let policy_hash = blake3::hash(&[&b"NONOS.linux.policy.v1"[..], policy].concat());
    let parts = [&b"NONOS.linux.app.v1"[..], &policy[16..48], policy_hash.as_bytes()];
    *blake3::hash(&parts.concat()).as_bytes()
}
