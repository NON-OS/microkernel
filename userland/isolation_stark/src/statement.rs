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

//! The eight public words and the private witness they come from.

use nonos_proof_abi::{allows, hist_commit, syscall_root, BUCKETS, HIST_WORDS, SET_BYTES};

/// What only the prover holds: the sealed histogram and its salt, and the
/// declared set, which is public but read here bucket by bucket.
#[derive(Clone)]
pub struct Witness {
    pub hist: [u32; HIST_WORDS],
    pub salt: [u64; 4],
    pub set: [u8; SET_BYTES],
}

/// the plan: Hc commits to the histogram, A to the set. No counts.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Statement {
    pub hc: [u64; 4],
    pub a: [u64; 4],
}

impl Statement {
    pub fn words(&self) -> [u64; 8] {
        let (h, a) = (self.hc, self.a);
        [h[0], h[1], h[2], h[3], a[0], a[1], a[2], a[3]]
    }

    /// The statement a witness proves, or `None` when it proves none: a salt
    /// word or a set bit out of range, or a success outside the set.
    pub fn of(w: &Witness) -> Option<Statement> {
        if (0..BUCKETS).any(|b| w.hist[2 * b] > 0 && !allows(&w.set, b)) {
            return None;
        }
        Some(Statement { hc: hist_commit(&w.hist, &w.salt)?, a: syscall_root(&w.set)? })
    }
}
