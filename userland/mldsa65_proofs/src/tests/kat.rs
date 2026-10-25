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

//! The published known answer. The reference project's metadata for this
//! source gives the SHA-256 of its first known-answer record; that record
//! is rebuilt here through nonos-mldsa65 alone, from the generator's seed
//! and DRBG, and must hash to the published value.

use crate::drbg::Drbg;
use crate::hex::upper;
use nonos_hash::sha256;
use nonos_mldsa65::{keypair, sign};

/// `nistkat-sha256` in third_party/pqclean/crypto_sign/ml-dsa-65/META.yml.
const PUBLISHED: &str = "7cb96242eac9907a55b5c84c202f0ebd552419c50b2e986dc2e28f07ecebf072";

pub fn first_record() -> String {
    let entropy: [u8; 48] = core::array::from_fn(|i| i as u8);
    let mut outer = Drbg::new(&entropy);
    let mut seed = [0u8; 48];
    outer.fill(&mut seed);
    let mut msg = vec![0u8; 33];
    outer.fill(&mut msg);
    let mut inner = Drbg::new(&seed);
    let mut rand = |b: &mut [u8]| {
        inner.fill(b);
        true
    };
    let (pk, sk) = keypair(&mut rand).expect("key pair");
    let sig = sign(&sk, &msg, &mut rand).expect("signature");
    let sm = [&sig[..], &msg].concat();
    let mut t = String::new();
    t += &format!("count = 0\nseed = {}\nmlen = 33\nmsg = {}\n", upper(&seed), upper(&msg));
    t += &format!("pk = {}\nsk = {}\n", upper(&pk), upper(sk.as_bytes()));
    t += &format!("smlen = {}\nsm = {}\n", sm.len(), upper(&sm));
    t
}

fn lower(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

#[test]
fn the_first_known_answer_hashes_to_the_published_digest() {
    assert_eq!(lower(&sha256(first_record().as_bytes())), PUBLISHED);
}
