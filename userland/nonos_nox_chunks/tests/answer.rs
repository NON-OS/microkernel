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

//! The escrow agent's sealed answer (plan/22 step 1, v1.4.24): it opens
//! under the buyer's key alone, to the offer the agent sealed, and that offer
//! passes check_offer before any funding.

#[path = "common/k.rs"]
mod k;
#[path = "common/seller.rs"]
mod seller;
#[path = "common/x.rs"]
mod x;

use k::{kec, K};
use nonos_nox_chunks::{check_offer, chunk_root, key_commit, open_answer, seal_answer};
use nonos_nox_chunks::{AnswerWrong, Offer, X25519};
use seller::{content, offer, KEY};
use x::D;

const BUYER: [u8; 32] = [0x0b; 32];
const AGENT: [u8; 32] = [0x0a; 32];

fn honest() -> Offer {
    let (hashes, ciphertext, cipher_root) = offer(&content(), None);
    Offer { cipher_root, key_commit: key_commit(&K, &KEY), hashes, ciphertext }
}

#[test]
fn an_answer_opens_under_the_buyers_key_to_an_offer_that_checks() {
    let sealed = seal_answer((&K, &D), &AGENT, &D.public(&BUYER), &honest()).unwrap();
    assert_eq!(sealed[..32], D.public(&AGENT));
    let o = open_answer((&K, &D), &BUYER, &sealed).unwrap();
    assert_eq!((&o, o.key_commit), (&honest(), kec(&[&KEY])));
    let c = content();
    let root = chunk_root(&K, &c).unwrap();
    assert_eq!(
        check_offer(&K, &root, c.len() as u64, &o.hashes, &o.ciphertext, &o.cipher_root),
        Ok(())
    );
}

#[test]
fn nothing_else_opens() {
    let sealed = seal_answer((&K, &D), &AGENT, &D.public(&BUYER), &honest()).unwrap();
    let open = |b: &[u8]| open_answer((&K, &D), &BUYER, b);
    assert_eq!(
        open_answer((&K, &D), &[0x0c; 32], &sealed),
        Err(AnswerWrong::Seal),
        "another buyer"
    );
    for at in [0, 31, 32, sealed.len() - 1] {
        let mut t = sealed.clone();
        t[at] ^= 1;
        assert!(open(&t).is_err(), "byte {at} changed");
    }
    assert_eq!(open(&sealed[..47]), Err(AnswerWrong::Short));
    assert_eq!(open(&[[0u8; 32].as_slice(), &[0; 16]].concat()), Err(AnswerWrong::LowOrder));
    assert_eq!(seal_answer((&K, &D), &AGENT, &[0; 32], &honest()), None, "a low-order buyer key");
}
