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

//! The bench's controller, brought up by the shipping bring-up and bound
//! the way setup binds one after a successful probe.

use core::cell::Cell;

use super::controller::{controller, LPSS_PHYS};
use crate::driver::Driver;
use crate::init::{bring_up, BusSetup};
use crate::regs::Regs;
use crate::transaction::{BusState, Reach};

/// Run the shipping bring-up against the attached core and bind the result
/// to `bound_addr`, the one address a client may then reach; zero means
/// setup bound no declared device and nothing is in reach.
pub fn driver(clock_hz: u32, bound_addr: u8) -> Result<Driver, &'static str> {
    let regs = Regs::new(0, 0x1000);
    let setup = BusSetup { clock_hz, lpss_base: Some(LPSS_PHYS), standard_mode: false };
    let init = bring_up(regs, setup)?;
    Ok(Driver {
        device_id: controller().device_id,
        pci_device: controller().pci_device,
        claim_epoch: 1,
        mmio_grant: 2,
        setup,
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
        bound_pin: 0,
        bound_info: 0,
        reach: Cell::new(Reach::declared([(bound_addr, 0x0001)])),
        acpi_hid_asks: Cell::new(0),
        bus: Cell::new(BusState::Ready),
        doorbell: None,
        regs,
    })
}
