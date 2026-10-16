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

//! Server flights rebuilt from the RFC 8448 trace, a message at a time.

use alloc::vec::Vec;

use super::super::cert_message_vectors::message;
use super::super::rfc8448_flight::{keyed, sealed};
use crate::fixtures::rfc8448::SERVER_HELLO_RECORD;
use crate::handshake_state::Progress;

pub(super) const HANDSHAKE: u8 = 22;

/// A Certificate handshake message carrying `der` alone.
pub(super) fn certificate_message(der: &[u8]) -> Vec<u8> {
    let body = message(&[der]);
    let mut out = vec![11, (body.len() >> 16) as u8, (body.len() >> 8) as u8, body.len() as u8];
    out.extend_from_slice(&body);
    out
}

/*
 * The server's flight with `parts` in place of the trace's messages, closed by
 * a Finished the server computes over exactly those parts: the attacker is the
 * server here, holds the handshake secret, and can always do that.
 */
pub(super) fn flight_of(parts: &[&[u8]]) -> Vec<u8> {
    let state = keyed(SERVER_HELLO_RECORD);
    let mut transcript = state.transcript.clone();
    for part in parts {
        transcript.push(part);
    }
    let mac = crate::finished_value::finished_value(&state.keys.server_secret, &transcript.digest())
        .expect("finished");
    let mut finished = vec![20, 0, 0, 32];
    finished.extend_from_slice(&mac);
    let mut all: Vec<&[u8]> = parts.to_vec();
    all.push(&finished);
    let joined = all.concat();
    sealed(&[&joined], HANDSHAKE)
}

pub(super) fn verdict(flight: &[u8]) -> Option<Vec<u8>> {
    let mut state = keyed(flight);
    assert_eq!(state.advance(flight), Progress::Complete(flight.len()));
    state.verify(b"", 0, false).map(|done| done.certificates)
}
