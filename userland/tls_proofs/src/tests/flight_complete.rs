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

//! A flight is complete when a whole server Finished has been decrypted.

use super::rfc8448_flight::{coalesced, keyed, messages, sealed};
use crate::fixtures::rfc8448::{SERVER_HELLO_RECORD, SERVER_MESSAGES};
use crate::handshake_state::{HandshakeState, Progress, Start};

/// Feed every prefix of `flight` to one state, as bytes arrive, and return
/// it: each proper prefix must read as incomplete.
fn trickle(flight: &[u8]) -> HandshakeState {
    let hello = SERVER_HELLO_RECORD.len();
    let client = super::rfc8448_flight::client();
    for cut in 0..hello {
        assert!(matches!(HandshakeState::begin(&client, &flight[..cut]), Start::Waiting));
    }
    let mut state = keyed(&flight[..hello]);
    for cut in hello..flight.len() {
        assert_eq!(state.advance(&flight[..cut]), Progress::Incomplete, "cut at {cut}");
    }
    assert_eq!(state.advance(flight), Progress::Complete(flight.len()));
    state
}

#[test]
fn one_coalesced_record_completes_at_its_last_byte_only() {
    let flight = coalesced();
    let state = trickle(&flight);
    assert_eq!(state.seq, 1, "the one record was opened once");
    for cut in SERVER_HELLO_RECORD.len()..flight.len() {
        assert_eq!(keyed(&flight).advance(&flight[..cut]), Progress::Incomplete, "fresh {cut}");
    }
}

#[test]
fn four_records_complete_at_the_last_byte_only() {
    let parts = messages(SERVER_MESSAGES);
    assert_eq!(parts.len(), 4, "EncryptedExtensions, Certificate, CertificateVerify, Finished");
    let flight = sealed(&parts, 22);
    let state = trickle(&flight);
    assert_eq!(state.seq, 4, "each record was opened once");
    /* Ending on the boundary before Finished is a whole record, not a flight. */
    let before_finished = sealed(&parts[..3], 22);
    assert_eq!(keyed(&flight).advance(&before_finished), Progress::Incomplete);
}
