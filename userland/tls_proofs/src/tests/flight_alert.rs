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

//! An alert from the server ends its flight, and names why.

use super::rfc8448_flight::{client, keyed, messages, sealed};
use crate::fixtures::rfc8448::SERVER_MESSAGES;
use crate::handshake_state::{HandshakeState, Progress, Refusal, Start};

#[test]
fn an_encrypted_alert_ends_the_flight_with_that_alert() {
    let flight = sealed(&[&[2, 42]], 21);
    let mut state = keyed(&flight);
    assert_eq!(state.advance(&flight), Progress::Alert(42), "bad_certificate");
    assert_eq!(state.alert(), Some(42));
    assert_eq!(state.answer(b"server", 0, b"GET").err(), Some(Refusal::Alert(42)));
}

#[test]
fn an_alert_after_some_messages_still_ends_it() {
    let parts = messages(SERVER_MESSAGES);
    let mut flight = sealed(&parts[..2], 22);
    let hs = super::rfc8448_flight::hs_keys();
    let (key, iv) = (&hs.server_key, &hs.server_iv);
    let alert = crate::record_seal::seal(hs.suite, key, iv, 2, 21, &[2, 48]).expect("seal");
    flight.extend_from_slice(&alert);
    assert_eq!(keyed(&flight).advance(&flight), Progress::Alert(48), "unknown_ca");
}

#[test]
fn a_plaintext_alert_before_any_server_hello_is_named() {
    let flight = [21, 3, 3, 0, 2, 2, 40];
    assert!(matches!(HandshakeState::begin(&client(), &flight), Start::Alert(40)));
}

#[test]
fn an_unfinished_flight_is_not_answered() {
    let parts = messages(SERVER_MESSAGES);
    let flight = sealed(&parts[..3], 22);
    let mut state = keyed(&flight);
    assert_eq!(state.advance(&flight), Progress::Incomplete);
    let refused = state.answer(b"server", 0, b"GET").err();
    assert_eq!(refused, Some(Refusal::Incomplete));
}
