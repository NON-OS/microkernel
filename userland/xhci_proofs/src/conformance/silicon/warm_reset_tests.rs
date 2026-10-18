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

//! A USB 3 link in Inactive comes back only through a warm reset, and a
//! connect that drops while it is debounced is no device.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use nonos_devmodel::{run, FakeBar};

use super::super::model::{controller, PORT1};
use super::super::port_device::reset_port1;
use crate::constants::{
    PORTSC_CCS, PORTSC_PED, PORTSC_PLS_SHIFT, PORTSC_PP, PORTSC_PRC, PORTSC_WPR,
};
use crate::error::XhciError;

#[test]
fn a_usb3_link_in_inactive_gets_a_warm_reset() {
    let bar = controller(0);
    let inactive = PORTSC_PP | PORTSC_CCS | (6 << PORTSC_PLS_SHIFT);
    bar.present32(PORT1, inactive);
    let warm = Arc::new(AtomicU32::new(0));
    let seen = warm.clone();
    // Acts only on a value it did not present itself: the driver's write.
    let shown = AtomicU32::new(inactive);
    let _dev = run(&bar, move |bar: &FakeBar| {
        let written = bar.wrote32(PORT1);
        if written == shown.load(Ordering::Relaxed) {
            return;
        }
        let next = if written & PORTSC_WPR != 0 {
            seen.store(1, Ordering::Relaxed);
            PORTSC_PP | PORTSC_CCS | PORTSC_PED | PORTSC_PRC
        } else {
            PORTSC_PP | PORTSC_CCS | PORTSC_PED
        };
        shown.store(next, Ordering::Relaxed);
        bar.present32(PORT1, next);
    });
    reset_port1(&bar, true).expect("the warm reset enables the port");
    assert_eq!(warm.load(Ordering::Relaxed), 1);
}

#[test]
fn a_connect_that_drops_during_the_debounce_is_no_device() {
    let bar = controller(0);
    bar.present32(PORT1, PORTSC_PP | PORTSC_CCS);
    let started = Instant::now();
    let _dev = run(&bar, move |bar: &FakeBar| {
        // Gone 40 ms in, before 100 ms of stable connect.
        if started.elapsed() > Duration::from_millis(40) {
            bar.present32(PORT1, PORTSC_PP);
        }
    });
    assert_eq!(reset_port1(&bar, false), Err(XhciError::NoDeviceOnPort));
}
