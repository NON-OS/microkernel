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

//! A verdict signed with the operator's own key verifies and names that key,
//! in either form, from any form of the key; a changed byte or another key
//! does not verify.

use super::fake::Fake;
use super::sealed_fixture::Sealed;
use super::signing::{check_signature, operator, public, signed};
use crate::exit;
use nonos_capsule_sign::{algs::AlgId, keys::write_seed};
use std::fs;

#[test]
fn a_verdict_signed_with_the_operators_key_verifies_and_names_it() {
    for json in [true, false] {
        let (s, dir) = (Sealed::dev(), tempfile::tempdir().unwrap());
        let (path, key) = operator(dir.path(), 42);
        let (code, _out, file) = signed(&s, &path, json, Fake::default());
        assert_eq!(code, exit::DEV_GO);
        assert!(fs::read_to_string(&file).unwrap().contains(&public(&key)));
        assert_eq!(check_signature(&file, &public(&key)), exit::GO, "json {json}");
    }
}

#[test]
fn every_form_of_the_key_is_the_same_key() {
    let (s, dir) = (Sealed::dev(), tempfile::tempdir().unwrap());
    let (raw, key) = operator(&dir.path().join("raw"), 11);
    let hexed = dir.path().join("hex.seed");
    fs::write(&hexed, hex::encode([11u8; 32]) + "\n").unwrap();
    let nonos = dir.path().join("nonos.seed");
    write_seed(&nonos, AlgId::Ed25519, &[11; 32]).unwrap();
    for path in [raw, hexed, nonos] {
        let (code, _out, file) = signed(&s, &path, true, Fake::default());
        assert_eq!(code, exit::DEV_GO, "{}", path.display());
        assert_eq!(check_signature(&file, &public(&key)), exit::GO, "{}", path.display());
    }
}

#[test]
fn a_changed_byte_or_another_key_does_not_verify() {
    let (s, dir) = (Sealed::dev(), tempfile::tempdir().unwrap());
    let (path, key) = operator(dir.path(), 42);
    let (_, _out, file) = signed(&s, &path, true, Fake::default());
    let other = public(&operator(&dir.path().join("other"), 43).1);
    assert_eq!(check_signature(&file, &other), exit::SIGNATURE_BAD);
    let mut b = fs::read(&file).unwrap();
    b[10] ^= 0x01;
    fs::write(&file, b).unwrap();
    assert_eq!(check_signature(&file, &public(&key)), exit::SIGNATURE_BAD);
}
