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

//! What the tool refuses before anything is signed: a document the client
//! would refuse, one quoted by another AK or for another challenge, a log
//! that does not replay, a machine that ran no release loader, and an
//! invalid host name, fleet or validity.

use nsh_attest::Refusal;
use nsh_enrol::Error;

use super::{inputs, request};
use crate::build::{Built, Spec};

fn refused(b: &Built, edit: fn(&mut nsh_enrol::Request<'_>)) -> Error {
    let (keys, loaders) = inputs();
    let mut req = request(b, &keys, &loaders);
    edit(&mut req);
    nsh_enrol::body(&req).expect_err("refused")
}

#[test]
fn a_document_the_client_refuses_is_refused() {
    let v4 = Spec { version: 4, ..Spec::default() }.build();
    assert_eq!(refused(&v4, |_| {}), Error::Doc(Refusal::Version(4)));
    let forged = Spec { bound_dma: Some((1, true, 4, 0)), ..Spec::default() }.build();
    assert_eq!(refused(&forged, |_| {}), Error::Doc(Refusal::Binding));
}

#[test]
fn another_ak_or_another_challenge_is_refused() {
    let b = Spec::default().build();
    assert_eq!(refused(&b, |r| r.ak[0] ^= 1), Error::Ak);
    assert_eq!(refused(&b, |r| r.challenge = [0x5A; 32]), Error::Challenge);
}

#[test]
fn a_log_that_does_not_replay_is_refused() {
    let b = Spec { pcr: vec![(4, [0x44; 32])], ..Spec::default() }.build();
    assert_eq!(refused(&b, |_| {}), Error::Replay(4));
}

#[test]
fn a_machine_that_ran_no_release_loader_is_refused() {
    let b = Spec::default().build();
    assert_eq!(refused(&b, |r| r.loaders = &[[0x77; 32]]), Error::NoLoader);
    assert_eq!(refused(&b, |r| r.loaders = &[]), Error::NoLoader);
}

#[test]
fn a_bad_host_fleet_or_validity_is_refused() {
    let b = Spec::default().build();
    assert_eq!(refused(&b, |r| r.host = b"DB.example"), Error::Value("host name"));
    assert_eq!(refused(&b, |r| r.fleet = [0; 16]), Error::Value("fleet id"));
    assert_eq!(refused(&b, |r| r.not_after_ms = r.now_ms), Error::Value("validity"));
}
