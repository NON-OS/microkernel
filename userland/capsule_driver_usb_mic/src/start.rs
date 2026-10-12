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
//! A stream begun and ended on the device: its capture alternate selected,
//! its rate set (rate.rs, driver.usb_audio0's), and its endpoint configured
//! on the host; alternate 0 again to end it, so the bandwidth goes back.

use nonos_usbnet::bus::Bus;
use nonos_usbnet::setup::Setup;
use nonos_usbnet::xhci::XhciBus;

use crate::bind::Mic;
use crate::rate::rate;
use crate::xhci::configure;

pub fn start(bus: &mut XhciBus, m: &Mic) -> Result<(), i32> {
    bus.control_out(Setup::set_interface(m.s.interface, m.s.alt), &[])?;
    let begun = rate(bus, &m.config, &m.s).map_err(|(_, e)| e);
    match begun.and_then(|()| configure(bus.xhci, bus.slot, &m.s)) {
        Ok(_) => Ok(()),
        // Alternate 0 again, so a refused stream holds no bandwidth.
        Err(e) => {
            let _ = stop(bus, m);
            Err(e)
        }
    }
}

pub fn stop(bus: &mut XhciBus, m: &Mic) -> Result<(), i32> {
    bus.control_out(Setup::set_interface(m.s.interface, 0), &[])
}
