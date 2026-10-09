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

//! Bringing a stalled bulk pipe back: the host side through driver.xhci0,
//! then the device's halt cleared over endpoint 0, which also sets its data
//! toggle back to zero (USB 2.0, 9.4.5; Linux usb_clear_halt).

use crate::bus::Bus;
use crate::setup::Setup;

const TO_ENDPOINT: u8 = 0x02;
const CLEAR_FEATURE: u8 = 0x01;
const ENDPOINT_HALT: u16 = 0;

pub fn clear_halt<B: Bus>(bus: &mut B, endpoint: u8) -> Result<(), i32> {
    bus.reset_bulk(endpoint & 0x80 != 0)?;
    let setup = Setup::new(TO_ENDPOINT, CLEAR_FEATURE, ENDPOINT_HALT, endpoint as u16);
    bus.control_out(setup, &[])
}
