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

//! A crypto pool that never answers costs a handshake one bounded wait. The
//! handshake is refused, never let through, and the refusal says why.

use super::answer_gateway::gateway_flight;
use super::fake_server::Wire;
use super::rfc8448_flight::{coalesced, keyed};
use crate::crypto_port::CRYPTO_WAIT_MS;
use crate::handshake_state::{Progress, Refusal, Unverified};
use crate::session::SessionError;
use crate::stream::connect_unauthenticated;

const NOW: u64 = 20261001000000;

#[test]
fn a_silent_pool_refuses_the_chain_after_one_wait() {
    let flight = gateway_flight();
    let mut state = keyed(&flight);
    assert_eq!(state.advance(&flight), Progress::Complete(flight.len()));
    let (before, start) = (nonos_libc::waits().0, nonos_libc::mk_uptime_ms());
    nonos_libc::silence(true);
    let refused = state.answer(b"nonos.software", NOW, b"GET / HTTP/1.1\r\n\r\n").err();
    let why = state.verify(b"nonos.software", NOW, true).err();
    nonos_libc::silence(false);
    assert_eq!(refused, Some(Refusal::TooSlow), "fails closed, named");
    assert_eq!(why, Some(Unverified::TooSlow), "and is told from a bad chain");
    let (after, named) = nonos_libc::waits();
    assert_eq!((after - before, named), (2, Some(CRYPTO_WAIT_MS)), "one wait per pass");
    let spent = nonos_libc::mk_uptime_ms() - start;
    assert!(spent >= 2 * CRYPTO_WAIT_MS as i64 && spent < 3 * CRYPTO_WAIT_MS as i64);
}

#[test]
fn the_signature_that_verifies_with_an_answer_is_too_slow_without_one() {
    let flight = coalesced();
    let mut state = keyed(&flight);
    assert_eq!(state.advance(&flight), Progress::Complete(flight.len()));
    nonos_libc::silence(true);
    let silent = state.verify(b"", 0, false).err();
    nonos_libc::silence(false);
    assert_eq!(silent, Some(Unverified::TooSlow), "no answer is no signature");
    assert!(state.verify(b"", 0, false).is_ok(), "the same flight, answered, verifies");
}

#[test]
fn a_stream_over_a_silent_pool_ends_too_slow() {
    nonos_libc::silence(true);
    let silent = connect_unauthenticated(&mut Wire::new(), b"relay.example").err();
    nonos_libc::silence(false);
    assert_eq!(silent, Some(SessionError::TooSlow));
    assert!(connect_unauthenticated(&mut Wire::new(), b"relay.example").is_ok());
}
