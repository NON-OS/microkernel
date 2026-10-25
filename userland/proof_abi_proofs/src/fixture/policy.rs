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

//! A LinuxAppPolicy as the plan lays it out in v1.1, and a reader of the
//! fields the verify order takes. The policy's real decoder is C's
//! nonos_linux_policy; this one reads only what the order needs.

use nonos_proof_abi::{PolicyView, SET_BYTES};

pub const PROGRAM: [u8; 32] = [0xA7; 32];
const SET: usize = 152;

/// A policy for PROGRAM under `listing` (zero for a vendor program) whose
/// set lets `allowed` succeed, with WAYLAND | MODELS_RO.
pub fn policy(listing: &[u8; 32], allowed: &[usize]) -> [u8; 256] {
    let mut p = [0u8; 256];
    p[..8].copy_from_slice(b"NLXPOL01");
    p[8..10].copy_from_slice(&1u16.to_le_bytes());
    p[16..48].copy_from_slice(&PROGRAM);
    p[48..56].copy_from_slice(&0x21u64.to_le_bytes());
    p[56..88].copy_from_slice(listing);
    if *listing != [0; 32] {
        p[88..120].copy_from_slice(&[0x33; 32]);
    }
    for &b in allowed {
        p[SET + b / 8] |= 1 << (b % 8);
    }
    p
}

pub fn decode_policy(b: &[u8]) -> Option<PolicyView> {
    if b.len() != 256 || b[..8] != *b"NLXPOL01" || b[8..10] != 1u16.to_le_bytes() {
        return None;
    }
    Some(PolicyView {
        program_blake3: b[16..48].try_into().ok()?,
        mask: u64::from_le_bytes(b[48..56].try_into().ok()?),
        listing_id: b[56..88].try_into().ok()?,
        syscall_set: b[SET..SET + SET_BYTES].try_into().ok()?,
    })
}

/// the plan's app_id, written out here apart from the crate's own, so the two
/// are held to each other: BLAKE3("NONOS.linux.app.v1" | program | policy_hash).
pub fn app_id(policy: &[u8; 256]) -> [u8; 32] {
    let policy_hash = blake3::hash(&[&b"NONOS.linux.policy.v1"[..], policy].concat());
    let parts = [&b"NONOS.linux.app.v1"[..], &policy[16..48], policy_hash.as_bytes()];
    *blake3::hash(&parts.concat()).as_bytes()
}
