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

//! The meter's FETCH 1 ESCROW request: one fresh buyer key per
//! offer, and a proof of work at ESCROW_POW_EFFORT, found by walking nonces
//! from a random start. A nonce whose challenge selects a HashX program the
//! reference refuses, or whose solutions all miss the effort, gives way to
//! the next, NONCE_TRIES at most; past that the job fails.

use nonos_equix::{HashX, Solution, Solver, MAX_SOLS};
use nonos_nox_chunks::{pow_challenge, pow_ok, PowField, X25519};
use nonos_nox_config::ESCROW_POW_EFFORT;
use nonos_settle_proto::Escrow;

use super::want::{Asked, Want};

pub const NONCE_TRIES: u32 = 256;

/// The request, or None when `draw` gives out or no nonce meets the effort.
pub fn escrow_request(
    x: &impl X25519,
    draw: &mut impl FnMut() -> Option<[u8; 32]>,
    solver: &mut Solver,
    want: &Want,
) -> Option<Asked> {
    let (mut buyer_sk, start) = (draw()?, draw()?);
    let buyer_pk = x.public(&buyer_sk);
    let mut nonce: [u8; 16] = start[..16].try_into().ok()?;
    for _ in 0..NONCE_TRIES {
        if let Some(solution) = solve(solver, &want.listing_key, &buyer_pk, &nonce) {
            let field = PowField { nonce, effort: ESCROW_POW_EFFORT, solution };
            let (chain_id, url) = (want.chain_id, want.url.clone());
            let (listing_key, release_key) = (want.listing_key, want.release_key);
            let solution = field.encode().to_vec();
            let request = Escrow { chain_id, url, listing_key, release_key, buyer_pk, solution };
            return Some(Asked { buyer_sk, request });
        }
        nonce = (u128::from_le_bytes(nonce).wrapping_add(1)).to_le_bytes();
    }
    buyer_sk.fill(0);
    None
}

fn solve(
    solver: &mut Solver,
    listing: &[u8; 32],
    pk: &[u8; 32],
    nonce: &[u8; 16],
) -> Option<[u8; 16]> {
    let c = pow_challenge(listing, pk, nonce, ESCROW_POW_EFFORT);
    let hashx = HashX::new(&c)?;
    let mut out = [Solution::default(); MAX_SOLS];
    let n = solver.solve(&hashx, &mut out);
    out[..n].iter().map(|s| s.to_bytes()).find(|s| pow_ok(&c, s, ESCROW_POW_EFFORT))
}
