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

//! A buyer as the meter is one: a key, a request with a proof of work at
//! ESCROW_POW_EFFORT built by settle's own encoder, and the release the agent
//! sells.

use nonos_equix::{HashX, Solution, Solver, MAX_SOLS};
use nonos_nox_chunks::{pow_challenge, pow_ok, PowField, X25519};
use nonos_nox_config::ESCROW_POW_EFFORT;
use nonos_settle_proto::Escrow;

use super::super::answer_types::Served;
use super::super::x::Dalek;

pub const LISTING: [u8; 32] = [1; 32];
pub const RELEASE: [u8; 32] = [2; 32];
pub const SK: [u8; 32] = [0x0b; 32];

pub fn served() -> Served {
    let content = (0..5000u32).map(|i| (i % 253) as u8).collect();
    Served { listing_key: LISTING, release_key: RELEASE, content }
}

/// The proof of work for buyer_pk at `effort`, walking nonces from zero.
pub fn pow(pk: &[u8; 32], effort: u32) -> PowField {
    let mut solver = Solver::new().unwrap();
    for i in 0u32..512 {
        let mut nonce = [0u8; 16];
        nonce[..4].copy_from_slice(&i.to_le_bytes());
        let c = pow_challenge(&LISTING, pk, &nonce, effort);
        let Some(h) = HashX::new(&c) else { continue };
        let mut out = [Solution::default(); MAX_SOLS];
        let n = solver.solve(&h, &mut out);
        if let Some(s) = out[..n].iter().map(|s| s.to_bytes()).find(|s| pow_ok(&c, s, effort)) {
            return PowField { nonce, effort, solution: s };
        }
    }
    panic!("no nonce in 512");
}

/// The request as settle sends it, at the agent's least effort.
pub fn request(chain_id: u64) -> Escrow {
    let buyer_pk = Dalek.public(&SK);
    let solution = pow(&buyer_pk, ESCROW_POW_EFFORT).encode().to_vec();
    let url = b"https://agent.example/e".to_vec();
    Escrow { chain_id, url, listing_key: LISTING, release_key: RELEASE, buyer_pk, solution }
}
