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

//! Handshake bytes, and fetches set up part way, for the fetch proofs.

use super::fetch_wire::FakeWire;
use crate::browser::fetch::run::run;
use crate::browser::fetch::types::{Fetch, Phase, TlsCtx};
use crate::browser::tls13::flight::ClientFlight;
use crate::browser::url::{self, Url};

/// RFC 8448's ClientHello and the client's X25519 private key, and the
/// server's answer: its ServerHello record and one encrypted record.
pub const RFC_CLIENT_HELLO: &[u8] = include_bytes!("fixtures/rfc8448_client_hello.tls");
pub const RFC_CLIENT_PRIVATE: &[u8] = include_bytes!("fixtures/rfc8448_client_private.tls");
pub const RFC_SERVER_HELLO: &[u8] = include_bytes!("fixtures/rfc8448_server_hello_record.tls");
pub const RFC_SERVER_FLIGHT: &[u8] = include_bytes!("fixtures/rfc8448_server_flight_record.tls");

/// The flight nonos.software's front end sent on the wire to port 49217:
/// ServerHello, ChangeCipherSpec and one 3,075 byte encrypted record.
pub const CAPTURED_FLIGHT: &[u8] = include_bytes!("fixtures/server_flight_49217.tls");

pub fn url_of(s: &str) -> Url {
    url::parse(s).expect("a test url parses")
}

pub fn rfc_client() -> ClientFlight {
    let mut private = [0u8; 32];
    private.copy_from_slice(RFC_CLIENT_PRIVATE);
    ClientFlight { record: Vec::new(), handshake: RFC_CLIENT_HELLO.to_vec(), private }
}

/// A fetch whose ClientHello `cf` has gone out, waiting for the flight.
pub fn awaiting_flight(target: &str, handle: u32, now: i64, cf: ClientFlight) -> Fetch {
    let mut f = Fetch::new(url_of(target), handle, Phase::TlsFlight, now);
    f.tls = Some(TlsCtx::new(cf, 20260601000000));
    f
}

/// One step of `f` with a tick's time to spend, at the fake clock's now.
pub fn step(w: &mut FakeWire, f: &mut Fetch) -> bool {
    let until = w.now.get() + 30;
    run(w, f, until)
}
