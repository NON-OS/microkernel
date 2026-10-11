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

//! A handshake has one end, HANDSHAKE_MS from its first byte, or the
//! caller's sooner end; a peer that sends a byte every 7 s, inside every
//! quiet window, is still cut off there. An honest peer finishes well
//! inside it. (The flight cap over many records is handshake_flight_cap.rs.)

use crate::session::{exchange, Io, SessionError, HANDSHAKE_MS};
use crate::stream::{connect, connect_unauthenticated, connect_until};
use nonos_libc::{mk_uptime_ms, skip_ms};

use super::fake_server::Wire;

/// The honest server, one byte per read, seven seconds apart.
struct Trickle(Wire);

impl Io for Trickle {
    fn write_all(&mut self, data: &[u8]) -> Result<(), SessionError> {
        self.0.write_all(data)
    }

    fn read(&mut self, into: &mut [u8]) -> Result<usize, SessionError> {
        let n = self.0.read(&mut into[..1])?;
        skip_ms(7_000 * n as i64);
        Ok(n)
    }
}

#[test]
fn a_trickling_peer_is_cut_off_at_the_deadline() {
    let t0 = mk_uptime_ms();
    let got = connect_unauthenticated(&mut Trickle(Wire::new()), b"relay");
    let spent = mk_uptime_ms() - t0;
    assert_eq!(got.err(), Some(SessionError::Handshake));
    assert!((HANDSHAKE_MS..HANDSHAKE_MS + 7_000 + 1_000).contains(&spent), "{spent} ms");
    let t0 = mk_uptime_ms();
    let got = exchange(&mut Trickle(Wire::new()), "relay", b"GET /", 0, 1 << 16);
    assert_eq!(got.err(), Some(SessionError::Handshake), "exchange holds the same end");
    assert!(mk_uptime_ms() - t0 < HANDSHAKE_MS + 8_000);
}

#[test]
fn a_callers_sooner_end_wins() {
    let t0 = mk_uptime_ms();
    let got = connect_until(&mut Trickle(Wire::new()), b"relay", 0, t0 + 10_000);
    assert_eq!(got.err(), Some(SessionError::Handshake));
    assert!(mk_uptime_ms() - t0 < 10_000 + 8_000, "stopped at the caller's end");
    let past = connect_until(&mut Wire::new(), b"relay", 0, mk_uptime_ms() - 1);
    assert_eq!(past.err(), Some(SessionError::Handshake), "an end already gone");
}

#[test]
fn an_honest_handshake_finishes_inside_the_deadline() {
    assert!(connect_unauthenticated(&mut Wire::new(), b"relay").is_ok());
    // The flight completes; only the chain, which no public root signs, fails.
    let until = mk_uptime_ms() + 5_000;
    let got = connect_until(&mut Wire::new(), b"relay", 1_780_000_000, until);
    assert_eq!(got.err(), Some(SessionError::Certificate));
    let got = connect(&mut Wire::new(), b"relay", 1_780_000_000);
    assert_eq!(got.err(), Some(SessionError::Certificate));
}
