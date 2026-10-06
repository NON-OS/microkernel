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

//! A refused flight names its certificate's problem, and still refuses.

use super::answer_gateway::gateway_flight;
use super::rfc8448_flight::{coalesced, keyed};
use crate::cert_problem::CertProblem;
use crate::fixtures::rfc8448::SERVER_HELLO_RECORD;
use crate::handshake_state::{Progress, Refusal};

const NOW: u64 = 20261001000000;

fn refused(host: &[u8], now: u64) -> Option<CertProblem> {
    let flight = gateway_flight();
    let mut state = keyed(&flight);
    assert_eq!(state.advance(&flight), Progress::Complete(flight.len()));
    let answer = state.answer(host, now, b"GET / HTTP/1.1\r\n\r\n").err();
    assert_eq!(answer, Some(Refusal::Unverified), "the reason never turns a refusal round");
    state.cert_problem(host, now)
}

#[test]
fn the_gateway_root_is_named_as_unknown() {
    assert_eq!(refused(b"nonos.software", NOW), Some(CertProblem::UnknownIssuer));
}
#[test]
fn the_wrong_host_is_named_as_a_mismatch() {
    assert_eq!(refused(b"example.com", NOW), Some(CertProblem::NameMismatch));
}
#[test]
fn a_clock_far_out_is_named_as_expired_or_early() {
    assert_eq!(refused(b"nonos.software", 20990101000000), Some(CertProblem::Expired));
    assert_eq!(refused(b"nonos.software", 20000101000000), Some(CertProblem::NotYetValid));
}
#[test]
fn only_a_flight_that_carried_a_certificate_has_a_reason() {
    let flight = coalesced();
    let mut state = keyed(&flight);
    let _ = state.advance(&flight);
    assert!(state.cert_problem(b"server", NOW).is_some(), "the trace's certificate is read");
    let empty = keyed(SERVER_HELLO_RECORD);
    assert_eq!(empty.cert_problem(b"server", NOW), None);
}
