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

//! A controller just brought up, as a driver: nothing bound yet, nothing in
//! reach, the bus ready and no doorbell. A bind (bind.rs) or the last
//! resort's reach (run.rs) fills in the rest.

use core::cell::Cell;

use crate::discover::Found;
use crate::driver::Driver;
use crate::init::{BusSetup, InitState};
use crate::regs::Regs;
use crate::transaction::{BusState, Reach};

/// The driver of `dev`, claimed at `claim_epoch`, its window mapped as
/// grant `mmio_grant` and reached through `regs`, brought up by `setup` to
/// `init`.
pub(super) fn unbound(
    dev: &Found,
    claim_epoch: u64,
    mmio_grant: u64,
    setup: BusSetup,
    init: InitState,
    regs: Regs,
) -> Driver {
    Driver {
        device_id: dev.device_id,
        pci_device: dev.pci_device,
        claim_epoch,
        mmio_grant,
        setup,
        family: dev.family,
        comp_type: init.comp_type,
        comp_param: init.comp_param,
        tx_depth: init.tx_depth,
        rx_depth: init.rx_depth,
        enabled: init.enabled,
        status: init.status,
        bound_by_probe: false,
        bound_addr: 0,
        bound_desc_reg: 0,
        bound_pin: 0,
        bound_info: 0,
        reach: Cell::new(Reach::default()),
        acpi_hid_asks: Cell::new(0),
        bus: Cell::new(BusState::Ready),
        doorbell: None,
        regs,
    }
}
