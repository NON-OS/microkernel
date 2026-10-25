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

//! An event: a PCR a PC TPM has, one digest under every declared bank, each
//! once, and a body within its cap, all inside the log.

use nsh_attest::log::replay;
use nsh_attest::LogFault;

use crate::build::tcg::{header, SHA1, SHA256};
use crate::build::tcg_event::{digest, ev, event};

fn one(e: Vec<u8>) -> Result<(), LogFault> {
    replay(&[header(), e].concat()).map(|_| ())
}

#[test]
fn a_pcr_past_23_is_refused() {
    assert!(one(ev(23, 1, digest(b"x"))).is_ok());
    assert_eq!(one(ev(24, 1, digest(b"x"))), Err(LogFault::PcrIndex));
    assert_eq!(one(ev(u32::MAX, 1, digest(b"x"))), Err(LogFault::PcrIndex));
}

#[test]
fn digests_not_exactly_the_declared_banks_are_refused() {
    let (d1, d2) = ((SHA1, vec![1; 20]), (SHA256, vec![2; 32]));
    let cases = [
        vec![d2.clone()],
        vec![d1.clone(), d2.clone(), d2.clone()],
        vec![d2.clone(), d2.clone()],
        vec![(0x000C, vec![3; 48]), d2.clone()],
        vec![d1.clone(), d1.clone()],
    ];
    for (i, digests) in cases.iter().enumerate() {
        assert_eq!(one(event(0, 1, digests, b"")), Err(LogFault::Bank), "case {i}");
    }
    assert!(one(event(0, 1, &[d2, d1], b"")).is_ok(), "any order");
}

#[test]
fn an_event_body_over_the_cap_is_refused() {
    let d = [(SHA1, vec![1; 20]), (SHA256, vec![2; 32])];
    assert!(one(event(0, 1, &d, &vec![0; 65_536])).is_ok());
    assert_eq!(one(event(0, 1, &d, &vec![0; 65_537])), Err(LogFault::EventTooLarge));
}

#[test]
fn an_event_cut_anywhere_is_refused() {
    let e = ev(4, 1, digest(b"x"));
    for cut in 1..e.len() {
        assert_eq!(one(e[..cut].to_vec()), Err(LogFault::Short), "{cut}");
    }
}
