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

//! The verifier at the shipped parameters.

use std::time::Instant;

use nonos_lock::argon2::{argon2id, Params};
use nonos_lock::record::PARAMS;
use nonos_lock::verify::{derive, make, matches};

const SALT: [u8; 16] = *b"0123456789abcdef";

#[test]
fn kernel_argon2id_matches_the_reference_vector() {
    let mut out = [0u8; 4];
    argon2id(b"", b"saltsalt", Params { m_kib: 64, t: 1, p: 2 }, &mut out, &mut || {}).unwrap();
    assert_eq!(out, [0x14, 0x84, 0x17, 0x19]);
}

#[test]
fn the_shipped_parameters_are_rfc9106_second_recommended() {
    assert_eq!(PARAMS, Params { m_kib: 65536, t: 3, p: 4 });
    assert_eq!(PARAMS, nonos_lock::argon2::RECOMMENDED);
}

#[test]
fn only_the_right_passphrase_matches() {
    let started = Instant::now();
    let rec = make(b"correct horse battery", &SALT, &mut || {}).unwrap();
    println!("argon2id m=64MiB t=3 p=4 on this host: {:?}", started.elapsed());
    assert_eq!(rec.failures, 0);
    assert!(matches(&rec, b"correct horse battery", &mut || {}).unwrap());
    for wrong in [&b"correct horse batterz"[..], b"correct horse battery ", b"correct horse", b""] {
        assert!(!matches(&rec, wrong, &mut || {}).unwrap(), "{wrong:?}");
    }
}

#[test]
fn the_salt_changes_the_tag() {
    let a = derive(b"passphrase", &SALT, Params { m_kib: 256, t: 1, p: 1 }, &mut || {}).unwrap();
    let mut other = SALT;
    other[15] ^= 1;
    let b = derive(b"passphrase", &other, Params { m_kib: 256, t: 1, p: 1 }, &mut || {}).unwrap();
    assert_ne!(a, b);
}

#[test]
fn a_record_checks_with_its_own_parameters() {
    let p = Params { m_kib: 512, t: 2, p: 2 };
    let tag = derive(b"twelve chars", &SALT, p, &mut || {}).unwrap();
    let rec = nonos_lock::Record { params: p, salt: SALT, tag, failures: 4 };
    assert!(matches(&rec, b"twelve chars", &mut || {}).unwrap());
    assert!(!matches(&rec, b"twelve charz", &mut || {}).unwrap());
}

#[test]
fn the_derivation_yields_between_segments() {
    let mut turns = 0;
    derive(b"passphrase", &SALT, Params { m_kib: 256, t: 2, p: 2 }, &mut || turns += 1).unwrap();
    assert_eq!(turns, 2 * 4 * 2);
}
