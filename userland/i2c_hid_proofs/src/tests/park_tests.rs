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

//! Every wait at start parks: looking for the controller, settling the pad
//! after power-on and waiting out its reset all sleep for a bounded time and
//! never yield in a loop, which on an idle machine is a spin.

use nonos_libc::{slept_ms, yields};

use super::fixture::rig;
use crate::i2c_client::resolve;
use crate::setup;

/// The lookup window: 100 lookups, 20 ms apart, no pause after the last.
const LOOKUP_WINDOW_MS: u64 = 99 * 20;

#[test]
fn with_no_controller_the_lookup_gives_up_after_a_bounded_parked_window() {
    let (y, s) = (yields(), slept_ms());
    assert!(resolve().is_none());
    assert_eq!(yields(), y, "the lookup yielded instead of sleeping");
    assert_eq!(slept_ms() - s, LOOKUP_WINDOW_MS, "the window is the attempts and the pauses");
}

#[test]
fn with_no_controller_setup_refuses_after_the_same_window_and_nothing_else() {
    let (y, s) = (yields(), slept_ms());
    assert!(setup::run().is_err(), "setup succeeded with no controller to talk to");
    assert_eq!(yields(), y);
    assert_eq!(slept_ms() - s, LOOKUP_WINDOW_MS);
}

#[test]
fn a_registered_controller_is_found_at_once_without_waiting() {
    let _r = rig();
    let s = slept_ms();
    assert!(resolve().is_some());
    assert_eq!(slept_ms(), s, "a controller that is there costs no pause");
}

#[test]
fn waking_the_pad_sleeps_and_never_yields() {
    let _r = rig();
    let (y, s) = (yields(), slept_ms());
    let state = setup::run().expect("setup");
    assert!(state.woke, "the pad was not woken");
    assert_eq!(yields(), y, "a wait at start yielded instead of sleeping");
    // At least the settle after power-on; the reset wait adds whatever polls
    // the pad needed before its zero-length report.
    assert!(slept_ms() - s >= 20, "the pad was not given its settle time");
    // Bounded: settle twice at most, and 64 reset polls 5 ms apart.
    assert!(slept_ms() - s <= 2 * 20 + 64 * 5, "slept {} ms", slept_ms() - s);
}
