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

//! The proof-of-work walk in slices: the meter hashes a
//! budget of Equi-X indices a tick and serves PAY between, so a metered app
//! never waits behind an escrow purchase. A nonce whose challenge selects a
//! HashX program the reference refuses, or whose solutions all miss the
//! effort, gives way to the next; past NONCE_TRIES the walk gives up and
//! the buyer key is wiped.

use super::walk_found::{Walked, NONCE_TRIES};
use super::want::Want;
use nonos_equix::{HashX, Solution, Solver, MAX_SOLS};
use nonos_nox_chunks::{pow_ok, X25519};
use nonos_nox_config::ESCROW_POW_EFFORT;

pub struct Walk {
    pub(super) buyer_sk: [u8; 32],
    pub(super) buyer_pk: [u8; 32],
    pub(super) nonce: [u8; 16],
    pub(super) tried: u32,
    pub(super) hashx: Option<HashX>,
    pub(super) want: Want,
}

impl Walk {
    pub fn new(x: &impl X25519, buyer_sk: [u8; 32], start: [u8; 16], want: Want) -> Self {
        let buyer_pk = x.public(&buyer_sk);
        Walk { buyer_sk, buyer_pk, nonce: start, tried: 0, hashx: None, want }
    }

    /// Hashes up to `budget` more indices of the current nonce's instance.
    pub fn step(&mut self, solver: &mut Solver, budget: u32) -> Walked {
        if self.tried >= NONCE_TRIES {
            self.buyer_sk.fill(0);
            return Walked::GaveUp;
        }
        let Some(h) = self.hashx.as_ref() else {
            let Some(h) = HashX::new(&self.challenge()) else { return self.next_nonce() };
            solver.start();
            self.hashx = Some(h);
            return Walked::Going;
        };
        if !solver.hash_some(h, budget) {
            return Walked::Going;
        }
        let mut out = [Solution::default(); MAX_SOLS];
        let n = solver.finish(&mut out);
        let c = self.challenge();
        let Some(solution) =
            out[..n].iter().map(|s| s.to_bytes()).find(|s| pow_ok(&c, s, ESCROW_POW_EFFORT))
        else {
            return self.next_nonce();
        };
        Walked::Found(self.found(solution))
    }
}
