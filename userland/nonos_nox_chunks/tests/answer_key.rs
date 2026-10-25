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

//! The sealed answer's key is tagged "NONOS.nox.escrow.answer.v1" and binds
//! both public keys; its nonce is zero and its AD listing_key | release_key.

#[path = "common/answer_fixture.rs"]
mod answer_fixture;
#[path = "common/answer_keys.rs"]
mod answer_keys;
#[path = "common/k.rs"]
mod k;
#[path = "common/seller.rs"]
mod seller;
#[path = "common/x.rs"]
mod x;

use answer_fixture::honest;
use answer_keys::{AGENT, BUYER, LISTING, RELEASE};
use k::{kec, K};
use nonos_nox_chunks::{seal_answer, ANSWER_TAG, X25519};
use x::D;

#[test]
fn the_key_is_tagged_and_the_ad_is_the_release() {
    assert_eq!(ANSWER_TAG, b"NONOS.nox.escrow.answer.v1");
    let (a, b) = (D.public(&AGENT), D.public(&BUYER));
    let key = kec(&[ANSWER_TAG, &D.dh(&AGENT, &b), &a, &b]);
    let plain = honest().encode();
    let mut out = vec![0u8; plain.len() + 16];
    nonos_seal::seal(&key, &[0; 12], &[LISTING, RELEASE].concat(), &plain, &mut out).unwrap();
    let sealed = seal_answer((&K, &D), &AGENT, &b, (&LISTING, &RELEASE), &honest()).unwrap();
    assert_eq!(sealed[..32], a);
    assert_eq!(sealed[32..], out[..]);
}
