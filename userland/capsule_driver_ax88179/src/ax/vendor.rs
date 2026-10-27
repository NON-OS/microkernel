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

//! The chip's registers over endpoint 0: a vendor request to the device,
//! bRequest the space (MAC or PHY). A MAC register is wValue, with wIndex
//! and wLength its width in bytes; a PHY register is wIndex, wValue the
//! PHY's address, two bytes little-endian. A read must bring every byte
//! asked for: a short one is a failure, never a value padded with zeros.

use nonos_usbnet::setup::{DIR_IN, TYPE_VENDOR};
use nonos_usbnet::xhci::E_IO;
use nonos_usbnet::{Bus, Setup};

use super::regs::{ACCESS_MAC, ACCESS_PHY, PHY_ADDR};

/// A read of `out.len()` bytes, at most 8, by vendor request `request`.
pub fn vendor_in<B: Bus>(
    bus: &mut B,
    request: u8,
    at: (u16, u16),
    out: &mut [u8],
) -> Result<(), i32> {
    let mut got = [0u8; 8];
    let want = out.len().min(got.len());
    let setup = Setup::new(DIR_IN | TYPE_VENDOR, request, at.0, at.1);
    let n = bus.control_in(setup, &mut got[..want])?;
    if n != out.len() {
        return Err(E_IO);
    }
    out.copy_from_slice(&got[..n]);
    Ok(())
}

pub fn mac_out<B: Bus>(bus: &mut B, reg: u16, data: &[u8]) -> Result<(), i32> {
    let setup = Setup::new(TYPE_VENDOR, ACCESS_MAC, reg, data.len() as u16);
    bus.control_out(setup, data)
}

pub fn phy_out<B: Bus>(bus: &mut B, reg: u16, value: u16) -> Result<(), i32> {
    let setup = Setup::new(TYPE_VENDOR, ACCESS_PHY, PHY_ADDR, reg);
    bus.control_out(setup, &value.to_le_bytes())
}

pub fn phy_in<B: Bus>(bus: &mut B, reg: u16) -> Result<u16, i32> {
    let mut v = [0u8; 2];
    vendor_in(bus, ACCESS_PHY, (PHY_ADDR, reg), &mut v)?;
    Ok(u16::from_le_bytes(v))
}
