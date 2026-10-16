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

//! Bringing a port up waits on PORTSC: up to 1.5 s for the connection to
//! settle, then up to 1 s for the reset to finish. A failed controller is
//! not waited on. One gone from the bus reads all ones, every port
//! connected and enabled with its reset done, so it would be addressed;
//! one with a Host Controller Error never finishes the reset. Either ends
//! the wait at once.

use std::time::{Duration, Instant};

use nonos_devmodel::{run, FakeBar};
use nonos_libc::slept_ms;

use super::model::{controller, CAP_LEN, PORT1};
use super::port_device::reset_port1;
use crate::constants::{PORTSC_CCS, PORTSC_PP, PORTSC_PR, USBSTS, USBSTS_HCE};
use crate::error::XhciError;

const STS: usize = CAP_LEN + USBSTS as usize;

#[test]
fn a_port_of_a_controller_gone_from_the_bus_is_not_brought_up() {
    let bar = controller(0);
    let _gone = run(&bar, |bar: &FakeBar| {
        bar.present32(STS, u32::MAX);
        bar.present32(PORT1, u32::MAX);
    });
    while bar.wrote32(STS) != u32::MAX || bar.wrote32(PORT1) != u32::MAX {
        std::thread::yield_now();
    }
    let slept = slept_ms();
    assert_eq!(reset_port1(&bar, false), Err(XhciError::ControllerDead));
    // Its connect holds, so a debounce that does not look waits 100 ms.
    assert_eq!(slept_ms() - slept, 0, "the debounce waited on a controller gone");
}

#[test]
fn a_controller_that_fails_during_a_port_reset_ends_the_wait_at_once() {
    let bar = controller(0);
    bar.present32(PORT1, PORTSC_PP | PORTSC_CCS);
    // Nothing else answers the port, so the driver's PR stays to be seen.
    let _failing = run(&bar, |bar: &FakeBar| {
        if bar.wrote32(PORT1) & PORTSC_PR != 0 {
            bar.present32(STS, USBSTS_HCE);
        }
    });
    let started = Instant::now();
    assert_eq!(reset_port1(&bar, false), Err(XhciError::ControllerDead));
    assert!(started.elapsed() < Duration::from_secs(1), "the reset's timeout ran out");
}
