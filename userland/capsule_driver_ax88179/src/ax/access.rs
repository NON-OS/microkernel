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

//! Register access as ax88179_read_cmd and ax88179_write_cmd do it: vendor
//! requests to the device, 0xC0 in and 0x40 out, values little endian.
//! Each failure carries the name of the step for the log.

use nonos_usbnet::setup::{DIR_IN, TYPE_VENDOR};
use nonos_usbnet::xhci::E_IO;
use nonos_usbnet::{Bus, Setup};

use super::phy_regs::PHY_ID;
use super::regs::{ACCESS_MAC, ACCESS_PHY};

/// A step that stopped: its name for the log, and the errno.
pub type Step = (&'static str, i32);
type Name = &'static str;
type Done = Result<(), Step>;

/// bRequest, wValue, wIndex.
pub type Request = (u8, u16, u16);

/// A short answer is an error: Linux would go on with zeros in the bytes
/// that did not come, and no register here means anything as zeros.
pub fn read<B: Bus>(bus: &mut B, r: Request, out: &mut [u8], what: Name) -> Done {
    match bus.control_in(Setup::new(DIR_IN | TYPE_VENDOR, r.0, r.1, r.2), out) {
        Ok(n) if n == out.len() => Ok(()),
        Ok(_) => Err((what, E_IO)),
        Err(e) => Err((what, e)),
    }
}

pub fn write<B: Bus>(bus: &mut B, r: Request, data: &[u8], what: Name) -> Done {
    let setup = Setup::new(TYPE_VENDOR, r.0, r.1, r.2);
    bus.control_out(setup, data).map_err(|e| (what, e))
}

/// A MAC register: wValue is the register, wIndex and wLength its width.
pub fn read_mac<B: Bus>(bus: &mut B, reg: u16, out: &mut [u8], what: Name) -> Done {
    let width = out.len() as u16;
    read(bus, (ACCESS_MAC, reg, width), out, what)
}

pub fn write_mac<B: Bus>(bus: &mut B, reg: u16, data: &[u8], what: Name) -> Done {
    write(bus, (ACCESS_MAC, reg, data.len() as u16), data, what)
}

/// A PHY register: wValue is the PHY address, wIndex the register
/// (ax88179_mdio_read, ax88179_mdio_write).
pub fn read_phy<B: Bus>(bus: &mut B, reg: u16, what: Name) -> Result<u16, Step> {
    let mut v = [0u8; 2];
    read(bus, (ACCESS_PHY, PHY_ID, reg), &mut v, what)?;
    Ok(u16::from_le_bytes(v))
}

pub fn write_phy<B: Bus>(bus: &mut B, reg: u16, value: u16, what: Name) -> Done {
    write(bus, (ACCESS_PHY, PHY_ID, reg), &value.to_le_bytes(), what)
}
