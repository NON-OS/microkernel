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
use crate::server::pump_rule::{started, still_running};

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
