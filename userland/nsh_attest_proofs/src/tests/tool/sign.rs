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

//! Signing: a body naming another authority is not signed, a certificate
//! changed after signing does not verify, the key file reads back as the
//! same key, and anything else is not taken for one.

use nsh_enrol::keys::PUBLIC_FILE_LEN;
use nsh_enrol::{AuthorityKey, Error};

use super::{inputs, key, request};
use crate::build::Spec;

fn cert() -> (Vec<u8>, Vec<u8>) {
    let b = Spec::default().build();
    let (keys, loaders) = inputs();
    let body = nsh_enrol::body(&request(&b, &keys, &loaders)).expect("a body");
    (body.clone(), nsh_enrol::sign(&body, key(), 0).expect("signed"))
}

fn public() -> [u8; PUBLIC_FILE_LEN] {
    key().public().try_into().expect("a public key")
}

#[test]
fn a_body_for_another_authority_is_not_signed() {
    let (mut body, _) = cert();
    let at = body.len() - 32;
    body[at] ^= 1;
    let r = nsh_enrol::sign(&body, key(), 0).map(|_| ());
    assert_eq!(r, Err(Error::Value("authority: the body names another key")));
    assert!(nsh_enrol::sign(&body[..10], key(), 0).is_err());
}

#[test]
fn a_certificate_changed_after_signing_does_not_verify() {
    let (body, cert) = cert();
    assert!(nsh_enrol::verify(&cert, &public()).is_ok());
    for at in [20, body.len() - 40, body.len() + 5, cert.len() - 5] {
        let mut c = cert.clone();
        c[at] ^= 1;
        assert!(nsh_enrol::verify(&c, &public()).is_err(), "byte {at}");
    }
    let mut other = public();
    other[0] ^= 1;
    assert_eq!(nsh_enrol::verify(&cert, &other).map(|_| ()), Err(Error::SelfCheck));
}

#[test]
fn a_key_file_reads_back_as_the_same_key_and_nothing_else_does() {
    let again = AuthorityKey::from_file(&key().to_file()).expect("reads");
    assert_eq!((again.id(), again.public()), (key().id(), key().public()));
    let mut f = key().to_file();
    assert_eq!(AuthorityKey::from_file(&f[1..]).map(|_| ()), Err(Error::KeyFile));
    f[4] = 2;
    assert_eq!(AuthorityKey::from_file(&f).map(|_| ()), Err(Error::KeyFile));
}
