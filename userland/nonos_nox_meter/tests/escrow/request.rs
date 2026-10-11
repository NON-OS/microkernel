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

//! The meter's escrow request: a fresh buyer key, and a 36-byte
//! proof of work pow_check takes at ESCROW_POW_EFFORT; else nothing.

use nonos_equix::Solver;
use nonos_nox_chunks::{pow_check, PowField, POW_FIELD_LEN, X25519};
use nonos_nox_config::ESCROW_POW_EFFORT;
use nonos_nox_meter::escrow::{escrow_request, Want, NONCE_TRIES};

use super::x::D;

pub fn want() -> Want {
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
    assert_eq!(r.solution.len(), POW_FIELD_LEN);
    assert!(pow_check(&[1; 32], &r.buyer_pk, &r.solution, ESCROW_POW_EFFORT), "the agent's check");
    let f = PowField::parse(&r.solution).unwrap();
    assert_eq!(f.effort, ESCROW_POW_EFFORT);
    assert!(r.encode().is_some(), "the request fits its wire");
}

#[test]
fn no_key_or_no_nonce_no_request() {
    let mut solver = Solver::new().unwrap();
    assert!(escrow_request(&D, &mut || None, &mut solver, &want()).is_none());
    let mut once = Some([5u8; 32]);
    assert!(
        escrow_request(&D, &mut || once.take(), &mut solver, &want()).is_none(),
        "no nonce start"
    );
    const { assert!(NONCE_TRIES >= 64, "room for effort 32's expected walk") };
}
