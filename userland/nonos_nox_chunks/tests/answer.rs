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

//! The escrow agent's sealed answer: it opens under the buyer's key
//! alone, for the release asked for alone, to the offer the agent sealed, and
//! that offer passes check_offer before ORDER_NEW.

#[path = "common/answer_fixture.rs"]
mod answer_fixture;
#[path = "common/answer_keys.rs"]
mod answer_keys;
#[path = "common/answer_len.rs"]
mod answer_len;
#[path = "common/k.rs"]
mod k;
#[path = "common/seller.rs"]
mod seller;
#[path = "common/x.rs"]
mod x;

use answer_fixture::honest;
use answer_keys::{AGENT, BUYER, LISTING, RELEASE};
use answer_len::len;
use k::{kec, K};
use nonos_nox_chunks::{check_offer, chunk_root, open_answer, seal_answer, AnswerWrong, X25519};
use seller::{content, KEY};
use x::D;

fn sealed() -> Vec<u8> {
    seal_answer((&K, &D), &AGENT, &D.public(&BUYER), (&LISTING, &RELEASE), &honest()).unwrap()
}

#[test]
fn an_answer_opens_under_the_buyers_key_to_an_offer_that_checks() {
    let s = sealed();
    assert_eq!(s[..32], D.public(&AGENT));
    let o = open_answer((&K, &D), &BUYER, ((&LISTING, &RELEASE), len()), &s).unwrap();
    assert_eq!((&o, o.key_commit), (&honest(), kec(&[&KEY])));
    let root = chunk_root(&K, &content()).unwrap();
    assert_eq!(check_offer(&K, &root, len(), &o.hashes, &o.ciphertext, &o.cipher_root), Ok(()));
}

#[test]
fn nothing_else_opens() {
    let s = sealed();
    let open = |sk: &[u8; 32], rel: &[u8; 32], n: u64, b: &[u8]| {
        open_answer((&K, &D), sk, ((&LISTING, rel), n), b)
    };
    assert_eq!(open(&[0x0c; 32], &RELEASE, len(), &s), Err(AnswerWrong::Seal), "another buyer");
    assert_eq!(open(&BUYER, &[0x23; 32], len(), &s), Err(AnswerWrong::Seal), "another release");
    assert_eq!(open(&BUYER, &RELEASE, len() + 1024, &s), Err(AnswerWrong::Body), "another length");
    for at in [0, 31, 32, s.len() - 1] {
        let mut t = s.clone();
        t[at] ^= 1;
        assert!(open(&BUYER, &RELEASE, len(), &t).is_err(), "byte {at} changed");
    }
    assert_eq!(open(&BUYER, &RELEASE, len(), &s[..47]), Err(AnswerWrong::Short));
    let low = [[0u8; 32].as_slice(), &[0; 16]].concat();
    assert_eq!(open(&BUYER, &RELEASE, len(), &low), Err(AnswerWrong::LowOrder));
    let to_zero = seal_answer((&K, &D), &AGENT, &[0; 32], (&LISTING, &RELEASE), &honest());
    assert_eq!(to_zero, None, "a low-order buyer key");
}
