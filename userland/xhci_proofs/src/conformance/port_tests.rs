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

//! Port reset: power, connect, reset, and the change bits acknowledged.

use nonos_devmodel::run;

use super::model::{controller, PORT1};
use super::port_device::{attached, reset_port1};
use super::port_settle::settles_enabled;
use crate::constants::{PORTSC_CCS, PORTSC_PP};
use crate::error::XhciError;

#[test]
fn a_port_is_powered_reset_and_left_enabled_with_its_change_acknowledged() {
    let bar = controller(0);
    let _device = run(&bar, attached(true));
    reset_port1(&bar, false).expect("reset completes");
    let shown = settles_enabled(&bar);
    assert!(shown, "the device shows the port enabled with the change acknowledged");
}

#[test]
fn a_port_with_nothing_attached_is_refused() {
    let bar = controller(0);
    assert_eq!(reset_port1(&bar, false), Err(XhciError::NoDeviceOnPort));
}

#[test]
fn a_reset_the_port_never_completes_times_out() {
    // Powered and connected, as bring-up leaves a port, so only the reset is
    // waited on: power-good is 20 ms of real time, which a model thread the
    // host did not schedule misses, and powering is the first test's.
    let bar = controller(0);
    bar.present32(PORT1, PORTSC_PP | PORTSC_CCS);
    let _device = run(&bar, attached(false));
    assert_eq!(reset_port1(&bar, false), Err(XhciError::PortResetTimeout));
}
