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

//! What the operator expects, and nothing more: an application in PCR 4
//! the operator did not list is refused and named, two release loaders that
//! both ran are refused, and the document must answer the challenge drawn
//! for these host keys, from this nonce. Hex is read as exact digits only.

use nsh_enrol::Error;

use super::{inputs, request};
use crate::build::tcg::EV_APP;
use crate::build::tcg_event::digest;
use crate::build::Spec;

#[test]
fn an_application_the_operator_did_not_expect_is_refused_and_named() {
    let mut s = Spec::default();
    s.log.ext(4, EV_APP, digest(b"something else"));
    let b = s.build();
    let (keys, loaders) = inputs();
    let r = nsh_enrol::body(&request(&b, &keys, &loaders));
    assert_eq!(r, Err(Error::Unexpected(digest(b"something else"))));
}

#[test]
fn two_release_loaders_that_both_ran_are_refused() {
    let mut s = Spec::default();
    s.log.ext(4, EV_APP, digest(b"an older loader"));
    let b = s.build();
    let (keys, one) = inputs();
    let two = [one[0], digest(b"an older loader")];
    let r = nsh_enrol::body(&request(&b, &keys, &two));
    assert_eq!(r, Err(Error::Value("loaders: more than one release loader ran")));
}

#[test]
fn the_challenge_names_the_host_keys_and_the_nonce() {
    let (keys, _) = inputs();
    let c = nsh_enrol::challenge(&[1; 32], &keys);
    assert_ne!(c, nsh_enrol::challenge(&[2; 32], &keys), "another nonce");
    let mut other = keys;
    other[40] ^= 1;
    assert_ne!(c, nsh_enrol::challenge(&[1; 32], &other), "other host keys");
    assert_eq!(c, nsh_enrol::challenge(&[1; 32], &keys));
}

#[test]
fn hex_takes_exact_digits_only() {
    assert_eq!(nsh_enrol::hex::array::<2>("0aFf", "x"), Ok([0x0a, 0xff]));
    for bad in ["+a0f", "0a0", "0a0f0", "0g0f", " 0a0f", "0a0f ", "-a0f"] {
        assert!(nsh_enrol::hex::array::<2>(bad, "x").is_err(), "{bad:?}");
    }
}
