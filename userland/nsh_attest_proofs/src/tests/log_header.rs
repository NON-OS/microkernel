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

//! The log's header: PCR 0, `EV_NO_ACTION`, a zero digest, and a Spec ID
//! body of exactly the TCG shape, declaring a SHA-256 bank of 32 bytes.

use nsh_attest::log::replay;
use nsh_attest::LogFault;

use crate::build::tcg::{header, header_with, spec_body, EV_NO_ACTION, SHA1, SHA256};

const STD: [(u16, u16); 2] = [(SHA1, 20), (SHA256, 32)];
const V: [u8; 4] = [0, 2, 0, 2];

fn fault(log: Vec<u8>) -> Result<(), LogFault> {
    replay(&log).map(|_| ())
}

fn with_body(body: Vec<u8>) -> Result<(), LogFault> {
    fault(header_with(0, EV_NO_ACTION, [0; 20], &body))
}

#[test]
fn a_header_alone_replays_to_zero() {
    let r = replay(&header()).expect("a log with no events");
    assert_eq!((r.events, r.pcrs, r.last_application), (0, [[0; 32]; 24], None));
}

#[test]
fn a_header_not_in_the_tcg_shape_is_refused() {
    let body = spec_body(&STD, V);
    assert_eq!(fault(header_with(1, EV_NO_ACTION, [0; 20], &body)), Err(LogFault::Header));
    assert_eq!(fault(header_with(0, 4, [0; 20], &body)), Err(LogFault::Header));
    assert_eq!(fault(header_with(0, EV_NO_ACTION, [1; 20], &body)), Err(LogFault::Header));
    let mut sig = body.clone();
    sig[13] = b'2';
    assert_eq!(with_body(sig), Err(LogFault::Header));
    for v in [[1, 2, 0, 2], [0, 1, 0, 2], [0, 2, 0, 0], [0, 2, 0, 3]] {
        assert_eq!(with_body(spec_body(&STD, v)), Err(LogFault::Header), "{v:?}");
    }
    assert_eq!(with_body([body, vec![0]].concat()), Err(LogFault::Header));
}

#[test]
fn a_header_with_bad_banks_is_refused() {
    let bad: [&[(u16, u16)]; 7] = [
        &[],
        &[(SHA256, 32); 9],
        &[(SHA256, 32), (SHA256, 32)],
        &[(SHA256, 20)],
        &[(SHA1, 20)],
        &[(SHA1, 0), (SHA256, 32)],
        &[(0x0D, 65), (SHA256, 32)],
    ];
    for banks in bad {
        assert_eq!(with_body(spec_body(banks, V)), Err(LogFault::Bank), "{banks:?}");
    }
}
