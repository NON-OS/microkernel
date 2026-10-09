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


//! Which HSDirs hold a service's descriptor (hs_get_responsible_hsdirs).
//!
//! Every HSDir has an index on a ring, derived from its Ed25519 identity and
//! the shared random value. The descriptor for each replica is stored at
//! another index, derived from the blinded key, and held by the next
//! `SPREAD_FETCH` distinct HSDirs going round from there.

extern crate alloc;

use alloc::vec::Vec;

use crate::crypto::keccak::sha3_256_parts;

/// hsdir_n_replicas and hsdir_spread_fetch, the fork's defaults; the live
/// consensus sets neither.
pub const REPLICAS: u64 = 2;
pub const SPREAD_FETCH: usize = 3;

/// Where replica `replica` of the descriptor lives on the ring.
pub fn store_index(blinded: &[u8; 32], replica: u64, period: u64, length: u64) -> [u8; 32] {
    sha3_256_parts(&[
        b"store-at-idx",
        blinded,
        &replica.to_be_bytes(),
        &length.to_be_bytes(),
        &period.to_be_bytes(),
    ])
}

/// Where an HSDir with Ed25519 identity `node` sits on the ring.
pub fn node_index(node: &[u8; 32], srv: &[u8; 32], period: u64, length: u64) -> [u8; 32] {
    sha3_256_parts(&[b"node-idx", node, srv, &period.to_be_bytes(), &length.to_be_bytes()])
}

/// The shared random value used when the consensus carries none
/// (compute_disaster_srv).
pub fn disaster_srv(period: u64, length: u64) -> [u8; 32] {
    sha3_256_parts(&[b"shared-random-disaster", &length.to_be_bytes(), &period.to_be_bytes()])
}

/// The responsible HSDirs, as positions into `nodes`, in the order the
/// replicas name them. `nodes` holds each HSDir's ring index.
pub fn responsible(nodes: &[[u8; 32]], blinded: &[u8; 32], period: u64, length: u64) -> Vec<usize> {
    let mut sorted: Vec<usize> = (0..nodes.len()).collect();
    sorted.sort_by(|a, b| nodes[*a].cmp(&nodes[*b]));
    let mut out: Vec<usize> = Vec::new();
    if sorted.is_empty() {
        return out;
    }
    for replica in 1..=REPLICAS {
        let target = store_index(blinded, replica, period, length);
        let start = sorted.iter().position(|i| nodes[*i] >= target).unwrap_or(0);
        let mut at = start;
        let mut added = 0usize;
        while added < SPREAD_FETCH {
            let node = sorted[at];
            if !out.contains(&node) {
                out.push(node);
                added += 1;
            }
            at = (at + 1) % sorted.len();
            if at == start {
                break;
            }
        }
    }
    out
}
