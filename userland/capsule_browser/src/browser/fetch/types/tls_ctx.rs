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

//! One connection's TLS state, from the ClientHello on.

use alloc::vec::Vec;

use crate::browser::tls13::flight::ClientFlight;
use crate::browser::tls13::{AppReader, HandshakeState, TrafficKeys};

pub struct TlsCtx {
    pub cf: ClientFlight,
    /* The server's handshake flight as it arrives; emptied once answered. */
    pub flight: Vec<u8>,
    pub now: u64,
    /* Keyed once, when the ServerHello is whole, and dropped once answered. */
    pub hs: Option<HandshakeState>,
    /* The server's application keys, once the handshake verified. */
    pub server_app: Option<TrafficKeys>,
    /* The response records opened so far, each once. */
    pub reader: AppReader,
}

impl TlsCtx {
    pub fn new(cf: ClientFlight, now: u64) -> Self {
        TlsCtx { cf, flight: Vec::new(), now, hs: None, server_app: None, reader: AppReader::new() }
    }

    /*
     * The handshake is over: the flight, the decrypted messages and the
     * X25519 private key are not needed again, and holding a private key
     * longer than its use is how it ends up somewhere it should not.
     */
    /// Keep only what reading and sending application records needs.
    pub fn settle(&mut self, app: TrafficKeys) {
        self.server_app = Some(app);
        self.hs = None;
        self.flight = Vec::new();
        self.cf.private = [0; 32];
        self.cf.handshake = Vec::new();
    }
}
