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

//! The RFC 8448 handshake keyed from its own private key, and repacked.

use alloc::vec::Vec;

use crate::fixtures::rfc8448::{CLIENT_HELLO, CLIENT_PRIVATE};
use crate::fixtures::rfc8448::{SERVER_FLIGHT_RECORD, SERVER_HELLO_RECORD};
use crate::flight::ClientFlight;
use crate::handshake_state::{HandshakeState, Start};
use crate::traffic_keys::TrafficKeys;

/// The trace's client: its ClientHello and its X25519 private key.
pub fn client() -> ClientFlight {
    let mut private = [0u8; 32];
    private.copy_from_slice(CLIENT_PRIVATE);
    ClientFlight { record: Vec::new(), handshake: CLIENT_HELLO.to_vec(), private }
}

/// ServerHello, then the encrypted flight as the trace sent it: one record.
pub fn coalesced() -> Vec<u8> {
    [SERVER_HELLO_RECORD, SERVER_FLIGHT_RECORD].concat()
}

/// The state keyed from the ServerHello at the front of `flight`.
pub fn keyed(flight: &[u8]) -> HandshakeState {
    match HandshakeState::begin(&client(), flight) {
        Start::Ready(state) => state,
        _ => panic!("the trace's ServerHello keys the handshake"),
    }
}

/// The server's handshake traffic keys.
pub fn hs_keys() -> TrafficKeys {
    keyed(SERVER_HELLO_RECORD).keys
}

/// Handshake messages cut apart by their own u24 lengths.
pub fn messages(bytes: &[u8]) -> Vec<&[u8]> {
    let mut out = Vec::new();
    let mut at = 0;
    while at + 4 <= bytes.len() {
        let len = u32::from_be_bytes([0, bytes[at + 1], bytes[at + 2], bytes[at + 3]]) as usize;
        out.push(&bytes[at..at + 4 + len]);
        at += 4 + len;
    }
    out
}

/// The ServerHello, then each part sealed as its own record under the
/// server's handshake keys with inner content type `kind`.
pub fn sealed(parts: &[&[u8]], kind: u8) -> Vec<u8> {
    let k = hs_keys();
    let mut out = SERVER_HELLO_RECORD.to_vec();
    for (seq, part) in parts.iter().enumerate() {
        let (key, iv) = (&k.server_key, &k.server_iv);
        let record = crate::record_seal::seal(k.suite, key, iv, seq as u64, kind, part);
        out.extend_from_slice(&record.expect("seal"));
    }
    out
}
