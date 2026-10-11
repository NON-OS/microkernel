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

//! The agent's answer opens under the buyer's key alone, to an offer that
//! checks against the release, and K decrypts it to the release's bytes;
//! each refusal names its cause.

use nonos_nox_chunks::{check_offer, chunk_root, key_commit, open_answer, open_delivery};

use super::super::answer::{answer, Refusal};
use super::super::seen::Seen;
use super::super::x::Dalek;
use super::buyer::{request, served, LISTING};
use crate::hosts::Host;

#[test]
fn an_answer_opens_checks_and_decrypts_to_the_release() {
    let (sk, req) = request(11_155_111);
    let mut n = 0u8;
    let mut draw = || {
        n += 1;
        [n.wrapping_mul(37); 32]
    };
    let s = served();
    let (sealed, k) = answer(&s, &mut Seen::default(), &req.agent_body(), &mut draw).unwrap();
    let o = open_answer((&Host, &Dalek), &sk, &sealed).unwrap();
    assert_eq!(o.key_commit, key_commit(&Host, &k));
    let (root, len) = (chunk_root(&Host, &s.content).unwrap(), s.content.len() as u64);
    assert_eq!(check_offer(&Host, &root, len, &o.hashes, &o.ciphertext, &o.cipher_root), Ok(()));
    let got = open_delivery(&Host, &k, &o.key_commit, &o.hashes, &o.ciphertext, len);
    assert_eq!(got.as_deref(), Ok(&s.content[..]), "K decrypts the release");
    assert!(open_answer((&Host, &Dalek), &[0x77; 32], &sealed).is_err(), "another buyer");
}

#[test]
fn each_refusal_names_its_cause() {
    let (_, req) = request(11_155_111);
    let s = served();
    let mut seen = Seen::default();
    let mut draw = || [9u8; 32];
    let mut ask = |body: &[u8]| answer(&s, &mut seen, body, &mut draw).map(|_| ());
    let body = req.agent_body();
    assert_eq!(ask(&body), Ok(()));
    assert_eq!(ask(&body), Err(Refusal::Seen), "one answer per solution");
    assert_eq!(ask(&body[1..]), Err(Refusal::Body));
    let mut other = body.clone();
    other[40] ^= 1;
    assert_eq!(ask(&other), Err(Refusal::NotServed), "another release_key");
    let mut wrong = body.clone();
    let last = wrong.len() - 1;
    wrong[last] ^= 1;
    assert_eq!(ask(&wrong), Err(Refusal::Solution));
    let mut chain = body;
    chain[..8].copy_from_slice(&5u64.to_le_bytes());
    assert_eq!(ask(&chain), Err(Refusal::Chain));
    assert_eq!(LISTING, s.listing_key);
}
