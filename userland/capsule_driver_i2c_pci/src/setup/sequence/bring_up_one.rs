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
use nonos_libc::{mk_device_release, mk_irq_ack};

use crate::discover::Found;
use crate::driver::Driver;
use crate::init::{bring_up, BusSetup};
use crate::regs::Regs;
use crate::setup::{claim, irq, mmio, pci};

use super::unlisted_gate::unlisted_gate;

pub(super) fn bring_up_one(dev: Found, standard_mode: bool) -> Result<Driver, &'static str> {
    let claim_epoch = claim::claim(dev.device_id)?;
    if !dev.is_acpi {
        pci::enable(dev.device_id, claim_epoch).map_err(|e| release(dev.device_id, e))?;
    }
    let mmio = mmio::map(dev, claim_epoch)?;
    let irq = irq::bind(dev, claim_epoch);
    let regs = Regs::new(mmio.user_va);
    unlisted_gate(&dev, regs).map_err(|e| release(dev.device_id, e))?;
    let setup = BusSetup {
        clock_hz: dev.clock_hz,
        lpss_base: dev.is_lpss().then_some(dev.bar0_base),
        standard_mode,
    };
    let init = bring_up(regs, setup).map_err(|e| release(dev.device_id, e))?;
    if irq.grant_id != 0 {
        let _ = mk_irq_ack(irq.grant_id);
    }
    Ok(Driver {
        device_id: dev.device_id,
        pci_device: dev.pci_device,
        claim_epoch,
        mmio_grant: mmio.grant_id,
        irq_grant: irq.grant_id,
        irq_vector: irq.vector,
        clock_hz: dev.clock_hz,
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
        doorbell: None,
        regs,
    })
}

/// Give the controller back when bringing it up fails, so its claim, BAR
/// mapping and IRQ binding do not outlive the attempt and the fallback loops
/// in `run.rs` can claim it again. The kernel cascades the teardown.
fn release(device_id: u64, why: &'static str) -> &'static str {
    let _ = mk_device_release(device_id);
    why
}
