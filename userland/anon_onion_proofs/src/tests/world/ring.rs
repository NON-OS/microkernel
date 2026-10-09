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


//! Where the service stores its descriptor: the HSDir ring as the service
//! side computes it (hs_get_responsible_hsdirs, store with the fetch index
//! at this point in the day), written with the sha3 crate.

use std::vec::Vec;

use sha3::{Digest, Sha3_256};

fn h(parts: &[&[u8]]) -> [u8; 32] {
    let mut d = Sha3_256::new();
    for p in parts {
        d.update(p);
    }
    d.finalize().into()
}

/// Positions into `ids` of the HSDirs responsible for `blinded`.
pub fn responsible(ids: &[[u8; 32]], srv: &[u8; 32], blinded: &[u8; 32], period: u64) -> Vec<usize> {
    let len = 1440u64.to_be_bytes();
    let p = period.to_be_bytes();
    let mut ring: Vec<([u8; 32], usize)> = ids.iter().enumerate().map(|(i, id)| (h(&[b"node-idx", id, srv, &p, &len]), i)).collect();
    ring.sort();
    let mut out = Vec::new();
    for replica in 1u64..=2 {
        let target = h(&[b"store-at-idx", blinded, &replica.to_be_bytes(), &len, &p]);
        let start = ring.iter().position(|(idx, _)| *idx >= target).unwrap_or(0);
        let mut taken = 0;
        let mut step = 0;
        while taken < 3 && step < ring.len() {
            let node = ring[(start + step) % ring.len()].1;
            if !out.contains(&node) {
                out.push(node);
                taken += 1;
            }
            step += 1;
        }
    }
    out
}
