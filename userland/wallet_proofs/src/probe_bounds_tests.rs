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
//! Every wait a refresh makes has a bound, read here from the constant the
//! code waits on, so a change there is checked here. A refresh is stepped a
//! tick at a time on the window's own thread (net/step/mod.rs):
//! - each step asks the route link once, for at most SLICE_MS;
//! - the stream opens within the route link's OPEN_MS;
//! - the handshake is cut HANDSHAKE_MS after the hello (handshake_end.rs);
//! - a proxy that leaves the request's frames unanswered is silent after
//!   SEND_WAIT_MS times ASKS (the route link's slice.rs);
//! - the answer's first piece waits the route's patience, each later piece
//!   the routed quiet, the whole answer the routed total (read_tls_flight.rs).
//!
//! A keyring call waits KEYRING_TIMEOUT_MS. The handshake's certificate check
//! asks net.crypto through nonos_tls with no bound yet; it joins here once it
//! has one.

use super::handshake_end::HANDSHAKE_MS;
use super::read_tls_flight::{routed, ANSWER_MAX, FLIGHT_MAX};
use super::step::slice::SLICE_MS;
use crate::keyring_wait::KEYRING_TIMEOUT_MS;
use crate::pick::Route;
use crate::route_bounds::{ASKS, OPEN_MS, SEND_WAIT_MS};

const ROUTES: [Route; 2] = [Route::Nym(1), Route::Anon(1)];

// A step holds the window for one slice at most, so input stays live.
const _: () = assert!(SLICE_MS > 0 && SLICE_MS <= 100, "a step leaves the window responsive");

// A wedged keyring returns control within seconds, and the bound leaves room
// for its slowest op: PBKDF2 with 2048 rounds plus BIP32 derivation.
const _: () = assert!(KEYRING_TIMEOUT_MS >= 2000 && KEYRING_TIMEOUT_MS <= 5000, "keyring");

// An answer, a batch of receipts with their logs, may run past a flight.
const _: () = assert!(FLIGHT_MAX <= ANSWER_MAX);

/// The longest a refresh takes on a host that stops answering at the worst
/// moment of each stage; None if the stages do not sum.
fn worst_case_ms(route: Route) -> Option<u64> {
    let silent = SEND_WAIT_MS.checked_mul(u64::try_from(ASKS).ok()?)?;
    let answer = routed(route.patience_ms()).total_ms;
    OPEN_MS.unsigned_abs().checked_add(HANDSHAKE_MS)?.checked_add(silent)?.checked_add(answer)
}

#[test]
fn a_refresh_on_a_dead_host_ends() {
    for route in ROUTES {
        assert!(worst_case_ms(route).is_some_and(|ms| ms > 0), "{} is bounded", route.label());
    }
}

#[test]
fn the_answer_outlasts_the_wait_for_its_first_piece() {
    // A reader that waits the route's whole patience for the first piece
    // still has time left to read the rest.
    for route in ROUTES {
        let b = routed(route.patience_ms());
        assert!(b.first_ms < b.total_ms && b.quiet_ms < b.total_ms, "{}", route.label());
    }
}
