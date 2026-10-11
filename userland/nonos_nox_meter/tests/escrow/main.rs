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

//! The meter's escrow request (plan/22 step 1): a fresh buyer key, its
//! public half in the request, and an Equi-X solution the agent's verify
//! accepts over listing_key | buyer_pk; another key when one cannot be solved.

use nonos_equix::{verify, Solution, Solver};
use nonos_nox_chunks::{equix_challenge, X25519};
use nonos_nox_meter::escrow::{escrow_request, Want, KEY_TRIES};
use x25519_dalek::{PublicKey, StaticSecret};

struct D;
impl X25519 for D {
    fn dh(&self, sk: &[u8; 32], pk: &[u8; 32]) -> [u8; 32] {
        StaticSecret::from(*sk).diffie_hellman(&PublicKey::from(*pk)).to_bytes()
    }
    fn public(&self, sk: &[u8; 32]) -> [u8; 32] {
        PublicKey::from(&StaticSecret::from(*sk)).to_bytes()
    }
}

fn want() -> Want {
    Want {
        chain_id: 11_155_111,
        url: b"https://agent.example/escrow".to_vec(),
        listing_key: [1; 32],
        release_key: [2; 32],
    }
}

#[test]
fn a_request_carries_the_buyer_key_and_a_solution_the_agent_accepts() {
    let mut solver = Solver::new().unwrap();
    let mut n = 0u8;
    let mut draw = || {
        n += 1;
        Some([n; 32])
    };
    let a = escrow_request(&D, &mut draw, &mut solver, &want()).unwrap();
    let r = &a.request;
    assert_eq!(r.buyer_pk, D.public(&a.buyer_sk));
    assert_eq!(
        (r.chain_id, &r.url[..], r.listing_key, r.release_key),
        (11_155_111, &want().url[..], [1; 32], [2; 32])
    );
    let sol = Solution::from_bytes(&r.solution[..].try_into().unwrap());
    assert_eq!(verify(&equix_challenge(&[1; 32], &r.buyer_pk), &sol), Ok(()));
    assert!(r.encode().is_some(), "the request fits its wire");
}

#[test]
fn no_key_no_request() {
    let mut solver = Solver::new().unwrap();
    assert!(escrow_request(&D, &mut || None, &mut solver, &want()).is_none());
    assert_eq!(KEY_TRIES, 8);
}
