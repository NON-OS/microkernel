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

//! Each refusal of the agent names its cause; a proof of work under the
//! least effort is refused as no solution.

use nonos_nox_config::ESCROW_POW_EFFORT;

use super::super::answer::answer;
use super::super::answer_types::Refusal;
use super::super::seen::Seen;
use super::buyer::{pow, request, served};

#[test]
fn each_refusal_names_its_cause() {
    let req = request(11_155_111);
    let s = served();
    let mut seen = Seen::default();
    let mut draw = || [9u8; 32];
    let mut ask = |body: &[u8]| answer(&s, &mut seen, body, 1, &mut draw).map(|_| ());
    let body = req.agent_body().expect("a request whose solution fits its u16");
    assert_eq!(ask(&body), Ok(()));
    assert_eq!(ask(&body), Err(Refusal::Seen), "one answer per proof of work");
    assert_eq!(ask(&body[1..]), Err(Refusal::Body));
    let mut other = body.clone();
    other[40] ^= 1;
    assert_eq!(ask(&other), Err(Refusal::NotServed), "another release_key");
    let mut wrong = body.clone();
    let last = wrong.len() - 1;
    wrong[last] ^= 1;
    assert_eq!(ask(&wrong), Err(Refusal::Solution));
    let mut cheap = req.clone();
    cheap.solution = pow(&req.buyer_pk, ESCROW_POW_EFFORT / 2).encode().to_vec();
    if cheap.solution != req.solution {
        assert_eq!(
            ask(&cheap.agent_body().expect("a request whose solution fits its u16")),
            Err(Refusal::Solution),
            "under the least effort"
        );
    }
    let mut chain = body;
    chain[..8].copy_from_slice(&5u64.to_le_bytes());
    assert_eq!(ask(&chain), Err(Refusal::Chain));
}
