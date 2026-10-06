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

//! A slow flight is progress, and a handshake that ran is not run again.

use super::fetch_fixtures::{
    awaiting_flight, rfc_client, step, RFC_SERVER_FLIGHT, RFC_SERVER_HELLO,
};
use super::fetch_wire::FakeWire;
use crate::browser::fetch::types::Phase;
use crate::browser::tls13;

/*
 * RFC 8448's flight, 48 bytes a second: seventeen seconds in all, past the
 * twelve second silence budget, with no gap near it. The deadline used to
 * run from the start of the fetch and ignore handshake bytes, so this timed
 * out, and the timeout was retried. The certificate chains to no root, so
 * the verdict is a refusal, reached once.
 */
#[test]
fn a_flight_trickling_past_the_silence_budget_is_answered_once() {
    let mut w = FakeWire::at(1_000);
    let mut f = awaiting_flight("https://server/", 11, 1_000, rfc_client());
    let flight = [RFC_SERVER_HELLO, RFC_SERVER_FLIGHT].concat();
    let chunks: Vec<&[u8]> = flight.chunks(48).collect();
    tls13::shim::reset();
    for (i, chunk) in chunks.iter().enumerate() {
        w.advance(1_000);
        w.deliver(11, chunk);
        step(&mut w, &mut f);
        let last = i + 1 == chunks.len();
        assert_eq!(f.phase == Phase::TlsFlight, !last, "chunk {i}: {:?}", f.error);
    }
    assert!(w.now.get() - 1_000 > 12_000, "the flight took longer than the budget");
    assert_eq!(f.error, Some("tls handshake refused"));
    let once = tls13::shim::counts();
    assert_eq!(once.x25519, 1, "keyed once, when the ServerHello was whole");
    for _ in 0..4 {
        w.advance(20_000);
        step(&mut w, &mut f);
    }
    assert_eq!(tls13::shim::counts(), once, "a handshake that ran is not run again");
    assert_eq!((f.error, w.opened.len()), (Some("tls handshake refused"), 0));
}
