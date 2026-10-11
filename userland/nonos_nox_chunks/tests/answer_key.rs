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

//! The sealed answer's key is tagged and binds both public keys; its nonce
//! is zero and its associated data buyer_pk | agent_pk (v1.4.24).

#[path = "common/k.rs"]
mod k;
#[path = "common/seller.rs"]
mod seller;
#[path = "common/x.rs"]
mod x;

use k::{kec, K};
use nonos_nox_chunks::{key_commit, seal_answer, Offer, ANSWER_TAG, X25519};
use seller::{content, offer, KEY};
use x::D;

const BUYER: [u8; 32] = [0x0b; 32];
const AGENT: [u8; 32] = [0x0a; 32];

fn honest() -> Offer {
    let (hashes, ciphertext, cipher_root) = offer(&content(), None);
    Offer { cipher_root, key_commit: key_commit(&K, &KEY), hashes, ciphertext }
}

#[test]
fn the_key_is_tagged_and_binds_both_public_keys() {
    let (a, b) = (D.public(&AGENT), D.public(&BUYER));
    let key = kec(&[ANSWER_TAG, &D.dh(&AGENT, &b), &a, &b]);
    let plain = honest().encode();
    let mut out = vec![0u8; plain.len() + 16];
    nonos_seal::seal(&key, &[0; 12], &[&b[..], &a].concat(), &plain, &mut out).unwrap();
    let sealed = seal_answer((&K, &D), &AGENT, &b, &honest()).unwrap();
    assert_eq!(sealed[32..], out[..], "nonce zero, AD buyer_pk | agent_pk");
}
