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

//! The audio server starts the driver's stream again when the driver says
//! it is not running (the lead's review, R17): a START refused before a
//! USB headset is bound is asked again on the next step, and a WRITE_PCM
//! answered E_NODEV marks the stream dropped. The driver's own E_NODEV is
//! the one the server tests for.

use crate::server::proto::{E_AGAIN, E_NODEV, E_OK};
use crate::server::pump_rule::{rechoose_now, started, still_running};

#[test]
fn a_refused_start_leaves_the_stream_stopped() {
    assert!(!started(false), "asked again on the next step");
    assert!(started(true));
}

#[test]
fn a_write_answered_no_device_drops_the_stream() {
    assert!(!still_running(E_NODEV));
    for s in [E_OK, E_AGAIN, -22, -5] {
        assert!(still_running(s), "{s}");
    }
    let driver = include_str!("../../capsule_driver_usb_audio/src/serve/answer.rs");
    assert!(driver.contains("play.device.is_none() {\n        E_NODEV"), "the driver's answer");
    assert_eq!(E_NODEV, -19);
}

/// A device lost mid-playback hands the output back to the fixed sink at
/// once, not at the next tidy tick: the pump notes the loss at both places
/// it reads a WRITE_PCM's answer, and the run loop chooses again on it.
#[test]
fn a_lost_device_is_fallen_back_from_at_once() {
    assert!(rechoose_now(false, true));
    assert!(rechoose_now(true, false));
    assert!(!rechoose_now(false, false));
    let pump = include_str!("../../capsule_audio/src/server/pump.rs");
    assert!(pump.contains("(state.running, state.lost) = (false, true);"));
    assert!(pump.contains("state.lost |= !state.running;"));
    let run = include_str!("../../capsule_audio/src/server/run.rs");
    assert!(run.contains(
        "if rechoose_now(now.wrapping_sub(last_reap) >= REAP_GAP_MS, pump.take_lost()) {"
    ));
    let state = include_str!("../../capsule_audio/src/server/pump_state.rs");
    assert!(state.contains("core::mem::take(&mut self.lost)"));
    let tidy = include_str!("../../capsule_audio/src/server/tidy.rs");
    assert!(tidy.contains("if sink.is_some_and(Sink::rechoose) {\n        pump.moved();"));
}
