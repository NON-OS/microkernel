// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use nonos_libc::mk_irq_ack;

use super::grants::{init_or_release, Grants};
use super::{claim, irq, mmio};
use crate::discover::Found;
use crate::driver::Driver;
use crate::firmware::family_for_device;
use crate::regs::Regs;

/// One bring-up attempt on the adapter discovery found. Each broker step
/// rolls back the ones before it when it fails, and a chip that will not
/// wake gives every grant back, so the next attempt can claim it again.
pub fn run(dev: Found) -> Result<Driver, &'static str> {
    family_for_device(dev.pci_device).ok_or("iwlwifi: unsupported device")?;
    let claim_epoch = claim::claim(dev.device_id)?;
    let mmio = mmio::map(dev, claim_epoch)?;
    let irq = irq::bind(dev, claim_epoch, &mmio)?;
    let grants =
        Grants { device_id: dev.device_id, claim_epoch, mmio: mmio.grant_id, irq: irq.grant_id };
    // SAFETY: the broker mapped `mmio.length` bytes of BAR0 at `user_va`
    // for this process, and they stay mapped while the claim is held.
    let regs = unsafe { Regs::new(mmio.user_va, mmio.length) };
    init_or_release(regs, &grants)?;
    let _ = mk_irq_ack(irq.grant_id);
    Ok(Driver {
        device_id: dev.device_id, pci_device: dev.pci_device, claim_epoch,
        mmio_grant: mmio.grant_id, mmio_len: mmio.length, irq_grant: irq.grant_id,
        regs,
    })
}
