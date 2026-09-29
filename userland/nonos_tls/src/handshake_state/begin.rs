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

//! Keying the handshake from the ServerHello, once.

use alloc::vec::Vec;

use super::types::{HandshakeState, Start};
use crate::flight::ClientFlight;
use crate::record_frame::record_at;

impl HandshakeState {
    /// Derive the handshake keys as soon as the first record is whole. The
    /// caller keeps the result: this is the step that costs a key agreement.
    pub fn begin(client: &ClientFlight, flight: &[u8]) -> Start {
        let Some((kind, _)) = record_at(flight, 0) else {
            return Start::Waiting;
        };
        if kind == crate::alert::ALERT {
            return match crate::alert::description_in_record(flight) {
                Some(description) => Start::Alert(description),
                None => Start::Unusable,
            };
        }
        /*
         * A retry is a ServerHello by type, so it is told apart before the key
         * share parse, which a retry's share-less extension would fail.
         */
        if crate::hello_retry::in_buffer(flight) {
            return Start::Retry;
        }
        match crate::server_keys::server_keys(client, flight) {
            Some(ctx) => Start::Ready(HandshakeState {
                keys: ctx.keys,
                transcript: ctx.transcript,
                cursor: ctx.used,
                seq: 0,
                msgs: Vec::new(),
                scanned: 0,
                end: None,
                alert: None,
                broken: false,
            }),
            None => Start::Unusable,
        }
    }
}
