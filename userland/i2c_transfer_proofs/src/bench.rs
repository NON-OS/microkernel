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

//! A driver over whatever core is attached on this thread, bound the way
//! setup binds one after a successful probe. Public so the HID proof crate
//! can stand a controller behind the HID driver's IPC calls.

use crate::driver::Driver;
use crate::init::{bring_up, BusSetup};
use crate::regs::Regs;

/// Gemini Lake's I2C input clock, the part this capsule was brought up on.
pub const GEMINI_LAKE_HZ: u32 = 133_000_000;
/// Where the controller's window sits physically, for the LPSS remap.
pub const LPSS_PHYS: u64 = 0x0000_0001_0000_0000;

/// Run the shipping bring-up against the attached core and bind the result
/// to `bound_addr`; zero means setup found no candidate and the HID driver
/// scans the bus itself.
pub fn driver(clock_hz: u32, bound_addr: u8) -> Result<Driver, &'static str> {
    let regs = Regs::new(0);
    let setup = BusSetup { clock_hz, lpss_base: Some(LPSS_PHYS), standard_mode: false };
    let init = bring_up(regs, setup)?;
    Ok(Driver {
        device_id: 1,
        pci_device: 0x31AC,
        claim_epoch: 1,
        mmio_grant: 2,
        irq_grant: 0,
        irq_vector: 0,
        clock_hz,
        family: "Gemini Lake",
        comp_type: init.comp_type,
        comp_param: init.comp_param,
        tx_depth: init.tx_depth,
        rx_depth: init.rx_depth,
        enabled: init.enabled,
        status: init.status,
        bound_by_probe: bound_addr != 0,
        bound_addr,
        bound_desc_reg: if bound_addr != 0 { 0x0001 } else { 0 },
        doorbell: None,
        regs,
    })
}
