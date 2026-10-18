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

//! A controller that failed is refused, not waited on: every request that
//! needs it answers E_NODEV at once, its ports show nothing attached, and
//! status requests are still served. Bringing it back is tried a bounded
//! number of times; this one's Host System Error survives the reset.

use std::time::{Duration, Instant};

use super::bench::Bench;
use super::hc_device::lock;
use crate::conformance::model::CAP_LEN;
use crate::constants::{USBSTS, USBSTS_HSE};
use crate::protocol::{
    E_NODEV, OP_CONTROLLER_STATUS, OP_ENABLE_SLOT, OP_GET_DEVICE_DESCRIPTOR, OP_PORT_STATUS,
};

const HIGH_SPEED: u32 = 3;

#[test]
fn a_controller_with_a_host_system_error_is_refused_and_shows_no_ports() {
    let mut b = Bench::new(HIGH_SPEED);
    let slot = b.root_device();
    lock(&b.hc).fault = true;
    let sts = CAP_LEN + USBSTS as usize;
    while b.bar.wrote32(sts) & USBSTS_HSE == 0 {
        std::thread::yield_now();
    }
    let started = Instant::now();
    assert_eq!(b.call(OP_GET_DEVICE_DESCRIPTOR, &[slot]).status, E_NODEV);
    assert!(started.elapsed() < Duration::from_secs(4), "no command or transfer ran out its time");
    let ports = b.call(OP_PORT_STATUS, &[]);
    assert_eq!((ports.status, ports.body[0]), (0, 2));
    for entry in ports.body[4..].chunks(8) {
        assert_eq!(entry[4..8], [0; 4], "port {}: nothing attached", entry[0]);
    }
    assert_eq!(b.call(OP_CONTROLLER_STATUS, &[]).status, 0, "status is still served");
    for _ in 0..4 {
        assert_eq!(b.call(OP_ENABLE_SLOT, &[]).status, E_NODEV);
    }
    assert_eq!(b.mux.ctxs[0].driver.revivals, 3, "brought back three times at most");
}
