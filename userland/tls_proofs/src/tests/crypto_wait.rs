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

//! A crypto pool that never answers costs a handshake one bounded wait, and
//! the handshake is refused, never let through.

use super::answer_gateway::gateway_flight;
use super::rfc8448_flight::{coalesced, keyed};
use crate::crypto_port::CRYPTO_WAIT_MS;
use crate::handshake_state::{Progress, Refusal};

const NOW: u64 = 20261001000000;

#[test]
fn a_silent_pool_refuses_the_chain_after_one_wait() {
    let flight = gateway_flight();
    let mut state = keyed(&flight);
    assert_eq!(state.advance(&flight), Progress::Complete(flight.len()));
    let (before, start) = (nonos_libc::waits().0, nonos_libc::mk_uptime_ms());
    nonos_libc::silence(true);
    let refused = state.answer(b"nonos.software", NOW, b"GET / HTTP/1.1\r\n\r\n").err();
    nonos_libc::silence(false);
    assert_eq!(refused, Some(Refusal::Unverified), "fails closed");
    let (after, named) = nonos_libc::waits();
    assert_eq!((after - before, named), (1, Some(CRYPTO_WAIT_MS)), "the first timeout ends it");
    let spent = nonos_libc::mk_uptime_ms() - start;
    assert!(spent >= CRYPTO_WAIT_MS as i64 && spent < 2 * CRYPTO_WAIT_MS as i64);
}

#[test]
fn the_signature_that_verifies_with_an_answer_fails_without_one() {
    let flight = coalesced();
    let mut state = keyed(&flight);
    assert_eq!(state.advance(&flight), Progress::Complete(flight.len()));
    nonos_libc::silence(true);
    let silent = state.verify(b"", 0, false);
    nonos_libc::silence(false);
    assert!(silent.is_none(), "no answer is no signature");
    assert!(state.verify(b"", 0, false).is_some(), "the same flight, answered, verifies");
    assert_eq!(nonos_libc::waits().1, Some(CRYPTO_WAIT_MS));
}
