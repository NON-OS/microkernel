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

//! The agent's answer opens under the buyer's key alone, for this release,
//! to an offer that checks, and K decrypts it to the release's bytes.

use nonos_nox_chunks::{check_offer, chunk_root, key_commit, open_answer, open_delivery};

use super::super::answer::answer;
use super::super::seen::Seen;
use super::super::x::Dalek;
use super::buyer::{request, served, LISTING, RELEASE, SK};
use crate::hosts::Host;

#[test]
fn an_answer_opens_checks_and_decrypts_to_the_release() {
    let req = request(11_155_111);
    let mut n = 0u8;
    let mut draw = || {
        n += 1;
        [n.wrapping_mul(37); 32]
    };
    let s = served();
    let len = s.content.len() as u64;
    let a = answer(
        &s,
        &mut Seen::default(),
        &req.agent_body().expect("a request whose solution fits its u16"),
        1,
        &mut draw,
    )
    .unwrap();
    let o = open_answer((&Host, &Dalek), &SK, ((&LISTING, &RELEASE), len), &a.sealed).unwrap();
    assert_eq!(
        (o.key_commit, a.key_commit, o.cipher_root),
        (key_commit(&Host, &a.k), o.key_commit, a.cipher_root)
    );
    let root = chunk_root(&Host, &s.content).unwrap();
    assert_eq!(check_offer(&Host, &root, len, &o.hashes, &o.ciphertext, &o.cipher_root), Ok(()));
    let got = open_delivery(&Host, &a.k, &o.key_commit, &o.hashes, &o.ciphertext, len);
    assert_eq!(got.as_deref(), Ok(&s.content[..]), "K decrypts the release");
    assert!(open_answer((&Host, &Dalek), &SK, ((&LISTING, &[3; 32]), len), &a.sealed).is_err());
}
