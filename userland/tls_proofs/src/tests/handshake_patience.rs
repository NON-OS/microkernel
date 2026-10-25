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

//! A transport's patience moves the handshake's end out by that much (A9h): a
//! first byte at 45 s over a 60 s route completes; a trickle is cut at the sum.

use crate::session::{exchange, Io, SessionError, HANDSHAKE_MS};
use crate::stream::connect_unauthenticated;
use nonos_libc::{mk_uptime_ms, skip_ms};

use super::super::fake_server::Wire;

const ROUTE_MS: i64 = 60_000;

/// A route of patience `.1`: empty 500 ms reads for `.2` ms, then a byte per `.3` ms.
struct Route(Wire, i64, i64, i64);

fn route(patience: i64, first: i64, per_byte: i64) -> Route {
    Route(Wire::new(), patience, first, per_byte)
}

impl Io for Route {
    fn write_all(&mut self, data: &[u8]) -> Result<(), SessionError> {
        self.0.write_all(data)
    }

    fn read(&mut self, into: &mut [u8]) -> Result<usize, SessionError> {
        if self.2 > 0 {
            let slice = self.2.min(500);
            self.2 -= slice;
            skip_ms(slice);
            return Ok(0);
        }
        let n = self.0.read(&mut into[..1])?;
        skip_ms(self.3 * n as i64);
        Ok(n)
    }

    fn patience_ms(&self) -> i64 {
        self.1
    }
}

#[test]
fn a_first_byte_at_45_s_over_a_60_s_route_completes() {
    assert!(connect_unauthenticated(&mut route(ROUTE_MS, 45_000, 10), b"relay").is_ok());
    let direct = connect_unauthenticated(&mut route(0, 45_000, 10), b"relay");
    assert_eq!(direct.err(), Some(SessionError::Handshake), "a socket keeps HANDSHAKE_MS");
    // The flight completes; only the chain, which no public root signs, fails.
    let got = exchange(&mut route(ROUTE_MS, 45_000, 10), "relay", b"GET /", 0, 1 << 16);
    assert_eq!(got.err(), Some(SessionError::Certificate), "exchange takes the same end");
}

#[test]
fn a_trickling_peer_is_still_cut_at_the_cap() {
    let t0 = mk_uptime_ms();
    let got = connect_unauthenticated(&mut route(ROUTE_MS, 0, 7_000), b"relay");
    let spent = mk_uptime_ms() - t0;
    assert_eq!(got.err(), Some(SessionError::Handshake));
    let cap = ROUTE_MS + HANDSHAKE_MS;
    assert!((cap..cap + 7_000 + 1_000).contains(&spent), "{spent} ms");
}
