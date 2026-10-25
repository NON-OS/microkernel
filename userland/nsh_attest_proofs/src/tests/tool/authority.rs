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

//! A certificate verifies only under the authority it names: one signed
//! by a key other than the one it names is refused even by that key, a key
//! whose halves do not belong together signs nothing, and a body that has
//! run out is not signed.

use nsh_crypto::{labelled, Label};
use nsh_enrol::keys::PUBLIC_FILE_LEN;
use nsh_enrol::Error;

use super::{inputs, request};
use crate::build::keys::{hybrid, AUTHORITY};
use crate::build::Spec;

fn body() -> Vec<u8> {
    let b = Spec::default().build();
    let (keys, loaders) = inputs();
    nsh_enrol::body(&request(&b, &keys, &loaders)).expect("a body")
}

#[test]
fn a_certificate_naming_another_authority_than_its_signer_does_not_verify() {
    let mut body = body();
    let signer = hybrid(AUTHORITY);
    let public: [u8; PUBLIC_FILE_LEN] =
        [&signer.ed.public[..], &signer.ml_public[..]].concat().try_into().expect("a key");
    let cert = [&body[..], &signer.sign(&labelled(Label::Enrol, &body))].concat();
    assert_eq!(nsh_enrol::verify(&cert, &public).map(|_| ()), Err(Error::SelfCheck));
    let at = body.len() - 32;
    body[at..].copy_from_slice(&nonos_hash::sha256(&public));
    let cert = [&body[..], &signer.sign(&labelled(Label::Enrol, &body))].concat();
    assert!(nsh_enrol::verify(&cert, &public).is_ok(), "named and signed by the same key");
}

#[test]
fn a_key_whose_halves_do_not_belong_together_signs_nothing() {
    let other = nsh_enrol::AuthorityKey::generate().expect("a key");
    let mut f = super::key().to_file();
    f[40..40 + 1952].copy_from_slice(&other.public()[32..]);
    let mixed = nsh_enrol::AuthorityKey::from_file(&f).expect("reads");
    let mut b = body();
    let at = b.len() - 32;
    b[at..].copy_from_slice(&mixed.id());
    assert_eq!(nsh_enrol::sign(&b, &mixed, 0).map(|_| ()), Err(Error::SelfCheck));
}

#[test]
fn a_body_that_has_run_out_is_not_signed() {
    let body = body();
    let e = nsh_wire::Enrol::parse(&[&body[..], &[0; 3373]].concat()).expect("parses").not_after_ms;
    assert!(nsh_enrol::sign(&body, super::key(), e - 1).is_ok());
    let r = nsh_enrol::sign(&body, super::key(), e).map(|_| ());
    assert_eq!(r, Err(Error::Value("validity: the body has run out")));
}
