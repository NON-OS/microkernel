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

//! The streaming alternate selected and its endpoint configured as one
//! step: a camera whose endpoint the host would not configure is put back
//! on alternate 0, as driver.usb_mic0 puts a microphone back, so a stream
//! that never began holds no bus bandwidth.

use nonos_usbnet::bus::Bus;
use nonos_usbnet::setup::Setup;

/// `alt` selected on `interface`, then `configure` run; on its failure,
/// alternate 0 again and the failure answered.
pub fn select<B: Bus>(
    bus: &mut B,
    interface: u8,
    alt: u8,
    configure: impl FnOnce(&mut B) -> Result<(), i32>,
) -> Result<(), i32> {
    bus.control_out(Setup::set_interface(interface, alt), &[])?;
    configure(bus).inspect_err(|_| {
        let _ = bus.control_out(Setup::set_interface(interface, 0), &[]);
    })
}
