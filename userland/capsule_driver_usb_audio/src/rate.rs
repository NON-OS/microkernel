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
//! A stream's rate set to 48 kHz: on its endpoint for USB Audio 1.0 where
//! it takes one, on its terminal's clock for 2.0, read back there, since a
//! 2.0 clock the host cannot set may run at another rate.

use nonos_usbnet::bus::Bus;
use nonos_usbnet::xhci::{XhciBus, E_INVAL};

use crate::uac::{clock, get_clock, select_first, set_clock, set_rate, Stream, RATE_HZ};

pub type Refusal = (&'static str, i32);

pub fn rate(bus: &mut XhciBus, config: &[u8], s: &Stream) -> Result<(), Refusal> {
    if s.version == 1 {
        if s.rate_control {
            let (setup, hz) = set_rate(s.ep_address, RATE_HZ);
            bus.control_out(setup, &hz).map_err(|e| ("set sampling rate", e))?;
        }
        return Ok(());
    }
    let c = clock(config, s.terminal).ok_or(("no clock for its terminal", E_INVAL))?;
    if let Some(selector) = c.selector {
        let (setup, pin) = select_first(&c, selector);
        bus.control_out(setup, &pin).map_err(|e| ("select clock", e))?;
    }
    if c.programmable {
        let (setup, hz) = set_clock(&c, RATE_HZ);
        bus.control_out(setup, &hz).map_err(|e| ("set clock rate", e))?;
    }
    let mut cur = [0u8; 4];
    let n = bus.control_in(get_clock(&c), &mut cur).map_err(|e| ("read clock rate", e))?;
    if n == 4 && u32::from_le_bytes(cur) == RATE_HZ {
        Ok(())
    } else {
        Err(("its clock does not run at 48 kHz", E_INVAL))
    }
}
