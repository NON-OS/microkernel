// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use nonos_libc::mk_irq_ack;

use super::grants::{init_or_release, Grants};
use super::{claim, dma, irq, mmio};
use crate::discover::Found;
use crate::driver::Driver;
use crate::firmware::family_for_device;
use crate::firmware::FirmwareStageState;
use crate::regs::Regs;

/// One bring-up attempt on the adapter discovery found. Each broker step
/// rolls back the ones before it when it fails, and a chip that will not
/// wake gives every grant back, so the next attempt can claim it again.
pub fn run(dev: Found) -> Result<Driver, &'static str> {
    let family = family_for_device(dev.pci_device).ok_or("iwlwifi: unsupported device")?;
    let claim_epoch = claim::claim(dev.device_id)?;
    let mmio = mmio::map(dev, claim_epoch)?;
    let irq = irq::bind(dev, claim_epoch, &mmio)?;
    let dma = dma::map_staging(dev.device_id, claim_epoch, &mmio, &irq)?;
    let grants = Grants {
        device_id: dev.device_id,
        claim_epoch,
        mmio: mmio.grant_id,
        irq: irq.grant_id,
        dma: dma.grant_id,
    };
    // SAFETY: the broker mapped `mmio.length` bytes of BAR0 at `user_va`
    // for this process, and they stay mapped while the claim is held.
    let regs = unsafe { Regs::new(mmio.user_va, mmio.length) };
    let init = init_or_release(regs, &grants)?;
    let _ = mk_irq_ack(irq.grant_id);
    Ok(Driver {
        device_id: dev.device_id, pci_device: dev.pci_device, claim_epoch,
        mmio_grant: mmio.grant_id, mmio_len: mmio.length, irq_grant: irq.grant_id,
        dma_grant: dma.grant_id, dma_user_va: dma.user_va,
        dma_device_addr: dma.device_addr, dma_len: dma.length,
        hw_rev: init.hw_rev, gp_cntrl: init.gp_cntrl, rf_kill: init.rf_kill, family, regs,
        firmware_stage: FirmwareStageState::empty(),
        rx_read_ptr: 0,
    })
}
