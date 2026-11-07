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

//! nsh.front serves one connection at a time, so a peer that connects and
//! never finishes the handshake must be let go at nsh.front's handshake
//! bound, long before its session bound, wherever in the handshake it
//! falls silent. The bounds are nsh.front's own, mounted from its source.

use super::front_limits::{HANDSHAKE_MS, SESSION_MAX_MS};

const _: () = assert!(
    HANDSHAKE_MS * 1000 < SESSION_MAX_MS,
    "the handshake bound is far inside the session bound"
);
use crate::kit::host::config;
use crate::kit::raw_run::RawRun;
use crate::kit::roles_build::NOW;
use nsh_server::{Event, Why};

/// Tick `r` with nothing more from the peer, to `at` after it connected.
fn silent_until(r: &mut RawRun, at: u64) {
    let rng = &mut r.rng;
    r.server.tick(NOW + at, &mut |b| rng.fill(b));
    r.events.extend(r.server.events());
}

fn let_go_at_the_bound(mut r: RawRun) {
    silent_until(&mut r, HANDSHAKE_MS - 1);
    assert!(!r.server.is_over(), "held until the bound");
    silent_until(&mut r, HANDSHAKE_MS);
    assert!(r.server.is_over(), "let go at the bound");
    assert!(r.events.contains(&Event::Refused(Why::HandshakeTime)));
}

#[test]
fn the_kit_holds_the_handshake_to_nsh_fronts_bound() {
    assert_eq!(config().handshake_ms, HANDSHAKE_MS);
}

#[test]
fn a_peer_that_says_nothing_is_let_go() {
    let_go_at_the_bound(RawRun::new());
}

#[test]
fn a_peer_that_stops_inside_its_hello_is_let_go() {
    let mut r = RawRun::new();
    let hello = r.raw.hello.clone();
    assert!(r.send(&hello[..hello.len() / 2]).is_empty());
    let_go_at_the_bound(r);
}

#[test]
fn a_peer_that_never_sends_user_auth_is_let_go() {
    let_go_at_the_bound(RawRun::proved());
}
