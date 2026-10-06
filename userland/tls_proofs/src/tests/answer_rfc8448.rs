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

//! One pass verifies the flight and seals the reply, as two passes did.

use alloc::vec::Vec;

use super::rfc8448_flight::{client, coalesced, keyed};
use crate::fixtures::rfc8448::{CLIENT_DATA_RECORD, CLIENT_FINISHED_RECORD};
use crate::fixtures::rfc8448::{SERVER_ALERT_RECORD, SERVER_DATA_RECORD, SERVER_TICKET_RECORD};
use crate::handshake_state::Progress;

pub(super) fn request() -> Vec<u8> {
    (0u8..50).collect()
}

/*
 * The trace's certificate chains to no root, so the success path is taken
 * with the chain check set aside, the way server_complete_unauthenticated
 * takes it; the signature and the Finished MAC are still checked.
 */
#[test]
fn the_reply_is_the_published_client_flight_and_the_two_pass_one() {
    let flight = coalesced();
    let mut state = keyed(&flight);
    assert_eq!(state.advance(&flight), Progress::Complete(flight.len()));
    let done = state.verify(b"", 0, false).expect("signature and Finished verify");
    let answer = crate::handshake_state::reply(done, &request()).expect("reply");
    assert_eq!(answer.flight, [CLIENT_FINISHED_RECORD, CLIENT_DATA_RECORD].concat());
    let two = crate::server_complete::server_complete_unauthenticated(&client(), &flight);
    let two = two.expect("the two-pass path agrees the flight verifies");
    let th = &two.transcript_hash;
    let mut legacy = crate::client_finished::client_finished(&two.handshake, th).expect("fin");
    let data = crate::application_request::application_request(&two.app, 0, &request());
    legacy.extend_from_slice(&data.expect("seal"));
    assert_eq!(answer.flight, legacy, "same client Finished and request");
    assert!(answer.app == two.app, "same application keys");
    let wire = [SERVER_TICKET_RECORD, SERVER_DATA_RECORD, SERVER_ALERT_RECORD].concat();
    let mut reader = crate::app_reader::AppReader::new();
    assert_eq!(reader.feed(&answer.app, &wire), 3, "ticket, data, close_notify");
    assert_eq!(reader.plaintext(), &request()[..], "only the data is application data");
}

#[test]
fn a_whole_verified_handshake_costs_one_kernel_round_trip() {
    let flight = coalesced();
    nonos_libc::reset();
    let mut state = keyed(&flight);
    assert_eq!(state.advance(&flight), Progress::Complete(flight.len()));
    let done = state.verify(b"", 0, false).expect("verifies");
    assert!(crate::handshake_state::reply(done, &request()).is_ok());
    let c = nonos_libc::counts();
    assert_eq!((c.kernel_round_trips, c.x25519), (1, 1), "the X25519 agreement alone");
    assert_eq!(c.rsa, 1, "the CertificateVerify signature");
    assert_eq!((c.p256, c.p384, c.sha384, c.other_ipc), (0, 0, 0, 0));
}
