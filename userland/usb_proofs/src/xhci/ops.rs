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

//! The controller driver's calls the hub and binding code makes. Every port
//! request goes to the hub in `sim`, a report descriptor read to
//! `report_desc`; a slot is always free, and addressing always works.

use super::{report_desc, sim, Setup};
use crate::hub::Place;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XhciClientError {
    Busy,
    Status(i32),
}

pub fn control_transfer(
    _xhci_port: u32,
    _slot: u8,
    setup: Setup,
    data: &mut [u8],
) -> Result<usize, XhciClientError> {
    let port = usize::from(setup.index);
    match (setup.request_type, setup.request) {
        // GET_STATUS of a port: wPortStatus then wPortChange.
        (0xA3, 0x00) => {
            data[..4].copy_from_slice(&sim::port_status(port));
            Ok(4)
        }
        // SET_FEATURE(PORT_RESET).
        (0x23, 0x03) if setup.value == 4 => {
            sim::reset(port);
            Ok(0)
        }
        // SET_PROTOCOL to an interface, STALLed as E_PIPE.
        (0x21, 0x0B) if report_desc::stalls_boot() => Err(XhciClientError::Status(-32)),
        // GET_DESCRIPTOR(Report) to an interface.
        (0x81, 0x06) if setup.value == 0x2200 => Ok(report_desc::give(data, setup.length)),
        _ => Ok(0),
    }
}

pub fn enable_slot(_xhci_port: u32) -> Result<u8, XhciClientError> {
    Ok(2)
}

pub fn disable_slot(_xhci_port: u32, _slot: u8) {}

pub fn address_routed(_xhci_port: u32, _slot: u8, _place: &Place) -> Result<(), XhciClientError> {
    sim::addressed();
    Ok(())
}
