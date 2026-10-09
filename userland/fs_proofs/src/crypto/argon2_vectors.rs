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

//! The kernel's Argon2 against RFC 9106 section 5 (all three types, which
//! between them walk both ways of addressing) and against an independent
//! Python reference for a long tag, a segment past one address block, and
//! a four-byte tag. The reference, `reference/argon2_fill.py`, prints the
//! same three RFC tags before the vectors copied here.

use super::argon2::{argon2, argon2id, Argon2Error, Inputs, Params, RECOMMENDED};
use super::test_input::unhex;

const RFC: [&str; 3] = [
    "512b391b6f1162975371d30919734294f868e3be3984f3c1a13a4db9fabe4acb",
    "c814d9d1dc7f37aa13f0d77f2494bda1c8de6b016dd388d29952a4c4672b6ce8",
    "0d640df58d78766c08c037a34a8b53c9d01ef0452d75b65eb52520e96b01e659",
];

#[test]
fn rfc9106_section_5_tags_for_argon2d_i_and_id() {
    let (pw, salt, secret, ad) = ([1u8; 32], [2u8; 16], [3u8; 8], [4u8; 12]);
    for (y, want) in RFC.iter().enumerate() {
        let inputs = Inputs { password: &pw, salt: &salt, secret: &secret, ad: &ad, y: y as u32 };
        let mut out = [0u8; 32];
        let mut segments = 0;
        argon2(&inputs, Params { m_kib: 32, t: 3, p: 4 }, &mut out, &mut || segments += 1).unwrap();
        assert_eq!(out.to_vec(), unhex(want), "type {y}");
        assert_eq!(segments, 3 * 4 * 4);
    }
}

fn id(pw: &[u8], salt: &[u8], m_kib: u32, t: u32, p: u32, len: usize) -> Vec<u8> {
    let mut out = vec![0u8; len];
    argon2id(pw, salt, Params { m_kib, t, p }, &mut out, &mut || {}).unwrap();
    out
}

#[test]
fn argon2id_matches_the_python_reference() {
    let long = "5db293b86f9252f52ca464bd1acb8ff6b9376f599d0649c2188ff9c1cd92c464\
        dda96dd966d906414fd997eb43daaa227617d32deb6d14e2a886632e055a54ee\
        6bfd9746d87d8cc8911c01da0f1739b8a5ab0ffa1a4ff7d21e1461de8765f944\
        6701800f";
    assert_eq!(id(b"password", b"somesaltsomesalt", 1024, 2, 1, 100), unhex(long));
    assert_eq!(id(b"", b"saltsalt", 64, 1, 2, 4), unhex("14841719"));
    let horse = "08c22821bdc48da963cda3d04c513aaf93ba828a9e170717091c96792495fea9\
        060cc882fd446fbdefe4875008dfb94fa681e48505bd130bb9a65d74d9d96f7b";
    assert_eq!(id(b"correct horse battery staple", &[0; 32], 256, 3, 4, 64), unhex(horse));
}

#[test]
fn parameters_outside_the_bounds_are_refused_before_any_memory_is_taken() {
    let mut out = [0u8; 32];
    let bad = [(7, 1, 1), (31, 1, 4), (64 * 1024 + 4, 1, 1), (64, 0, 1), (64, 17, 1), (64, 1, 0)];
    for (m_kib, t, p) in bad {
        let r = argon2id(b"pw", b"saltsalt", Params { m_kib, t, p }, &mut out, &mut || {});
        assert_eq!(r, Err(Argon2Error::BadParams), "m {m_kib} t {t} p {p}");
    }
    let r = argon2id(b"pw", b"saltsalt", RECOMMENDED, &mut [0u8; 3], &mut || {});
    assert_eq!(r, Err(Argon2Error::BadParams));
}
