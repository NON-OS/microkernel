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

//! A buyer as the meter is one: a key, a solved request built by settle's
//! own encoder, and the release the agent sells.

use nonos_equix::{HashX, Solution, Solver, MAX_SOLS};
use nonos_nox_chunks::{equix_challenge, X25519};
use nonos_settle_proto::Escrow;

use super::super::answer::Served;
use super::super::x::Dalek;

pub const LISTING: [u8; 32] = [1; 32];
pub const RELEASE: [u8; 32] = [2; 32];

pub fn served() -> Served {
    let content = (0..5000u32).map(|i| (i % 253) as u8).collect();
    Served { listing_key: LISTING, release_key: RELEASE, content }
}

/// A buyer key with a solvable challenge, and its request as settle sends it.
pub fn request(chain_id: u64) -> ([u8; 32], Escrow) {
    let mut solver = Solver::new().unwrap();
    for seed in 1..=8u8 {
        let sk = [seed; 32];
        let pk = Dalek.public(&sk);
        let Some(h) = HashX::new(&equix_challenge(&LISTING, &pk)) else { continue };
        let mut out = [Solution::default(); MAX_SOLS];
        if solver.solve(&h, &mut out) == 0 {
            continue;
        }
        let url = b"https://agent.example/e".to_vec();
        let solution = out[0].to_bytes().to_vec();
        let (listing_key, release_key, buyer_pk) = (LISTING, RELEASE, pk);
        return (sk, Escrow { chain_id, url, listing_key, release_key, buyer_pk, solution });
    }
    panic!("no solvable key in eight");
}
