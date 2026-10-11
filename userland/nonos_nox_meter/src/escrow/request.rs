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

//! The meter's FETCH 1 ESCROW request (plan/22 step 1, v1.4.24): a fresh
//! buyer key for the one offer, and one Equi-X solution over listing_key |
//! buyer_pk. A key whose challenge has no solution, or a HashX program the
//! reference refuses, is dropped for another, KEY_TRIES at most.

use alloc::vec::Vec;

use nonos_equix::{HashX, Solution, Solver, MAX_SOLS};
use nonos_nox_chunks::{equix_challenge, X25519};
use nonos_settle_proto::Escrow;

pub const KEY_TRIES: usize = 8;

/// What the request is for: the chain, the release's https endpoint, and its keys.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Want {
    pub chain_id: u64,
    pub url: Vec<u8>,
    pub listing_key: [u8; 32],
    pub release_key: [u8; 32],
}

pub struct Asked {
    pub buyer_sk: [u8; 32],
    pub request: Escrow,
}

/// The request, or None once `draw` gives out or KEY_TRIES keys fail.
pub fn escrow_request(
    x: &impl X25519,
    draw: &mut impl FnMut() -> Option<[u8; 32]>,
    solver: &mut Solver,
    want: &Want,
) -> Option<Asked> {
    for _ in 0..KEY_TRIES {
        let mut sk = draw()?;
        let pk = x.public(&sk);
        if let Some(solution) = solve(solver, &equix_challenge(&want.listing_key, &pk)) {
            let request = Escrow {
                chain_id: want.chain_id,
                url: want.url.clone(),
                listing_key: want.listing_key,
                release_key: want.release_key,
                buyer_pk: pk,
                solution: solution.to_bytes().to_vec(),
            };
            return Some(Asked { buyer_sk: sk, request });
        }
        sk.fill(0);
    }
    None
}

fn solve(solver: &mut Solver, challenge: &[u8]) -> Option<Solution> {
    let hashx = HashX::new(challenge)?;
    let mut out = [Solution::default(); MAX_SOLS];
    let n = solver.solve(&hashx, &mut out);
    out[..n].first().copied()
}
