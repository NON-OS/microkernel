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

//! The connection debounce on a controller with a Host Controller Error:
//! the error ends the wait at once, rather than after 100 ms of a connect
//! that holds, or up to 1.5 s of one that keeps bouncing, before a port
//! reset the failed controller will never finish. Registers only, no model:
//! nothing moves while the driver looks.

use nonos_devmodel::FakeBar;
use nonos_libc::slept_ms;

use super::model::{controller, CAP_LEN, PORT1};
use super::port_device::reset_port1;
use crate::constants::{PORTSC_CCS, PORTSC_PP, USBSTS, USBSTS_HCE};
use crate::error::XhciError;

fn failed_with_a_connect() -> std::sync::Arc<FakeBar> {
    let bar = controller(0);
    bar.present32(CAP_LEN + USBSTS as usize, USBSTS_HCE);
    bar.present32(PORT1, PORTSC_PP | PORTSC_CCS);
    bar
}

#[test]
fn a_connect_on_a_controller_with_a_host_controller_error_is_not_debounced() {
    let bar = failed_with_a_connect();
    let slept = slept_ms();
    assert_eq!(reset_port1(&bar, false), Err(XhciError::ControllerDead));
    assert_eq!(slept_ms() - slept, 0, "debounced a failed controller's port");
}
