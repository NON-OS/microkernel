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

//! One end for the whole TLS handshake (A9e's rule in the wallet's stepped
//! client): a host that trickles its flight a byte at a time, each inside
//! the routed quiet window, is cut at HANDSHAKE_MS from the hello, long
//! before the gather's own 90 s total would end it.

use crate::wallet::net::bounds::Bounds;
use crate::wallet::net::handshake_end::{end_at, over, HANDSHAKE_MS};
use crate::wallet::net::step::gather::{Gather, Gathered};

/* The routed flight's bounds, as read_tls_flight::routed gives them. */
const ROUTED: Bounds =
    Bounds { first_ms: 30_000, quiet_ms: 8_000, total_ms: 90_000, max: 24 * 1024 };
const SLICE: u64 = 50;

#[test]
fn a_trickling_host_is_cut_at_the_handshake_end() {
    let hello = 1_000u64;
    let end = Some(end_at(hello));
    /* The hello takes two seconds to go; the flight then comes a byte every 7 s. */
    let mut g = Gather::new(ROUTED, (hello + 2_000) as i64);
    let mut now = hello + 2_000;
    let mut next_byte = now + 7_000;
    while !over(end, now) {
        let byte: &[u8] = if now >= next_byte { b"x" } else { b"" };
        if now >= next_byte {
            next_byte += 7_000;
        }
        assert_eq!(g.take(byte, false, now as i64, |_| false), Gathered::More, "at {now}");
        now += SLICE;
    }
    assert_eq!(now, hello + HANDSHAKE_MS, "cut exactly at the end");
    assert_eq!(
        g.take(b"", false, now as i64, |_| false),
        Gathered::More,
        "the gather alone waits on"
    );
}

#[test]
fn the_end_holds_to_the_millisecond_and_never_wraps() {
    let end = Some(end_at(5));
    assert!(!over(end, 5 + HANDSHAKE_MS - 1) && over(end, 5 + HANDSHAKE_MS));
    assert!(!over(None, u64::MAX), "no hello yet, no end");
    assert_eq!(end_at(u64::MAX), u64::MAX, "saturates");
    assert_eq!(HANDSHAKE_MS, 30_000);
}
