// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Bind the adapter's interrupt: legacy INTx when firmware routed a line, else
//! one MSI-X vector (the 9000, AX200 and AX210 parts carry an MSI-X
//! capability), else none at all. The 7265 and 8265 are MSI-only, which the
//! broker does not offer, so on a machine whose firmware left their INTx line
//! unrouted they run polled. Every wait in this driver is a bounded register
//! poll, so polled mode loses latency, not function. Only a refusal that says
//! the claim is gone ends the attempt, giving back the register mapping and
//! the claim.

use nonos_libc::{
    mk_device_release, mk_irq_bind, mk_mmio_unmap, IrqBindOut, MmioMapOut, MK_IRQ_BIND_MSIX,
};

use super::irq_plan::{after_intx, after_msix, Refusal};
use crate::discover::Found;
use crate::pci_match::intx_usable;

/// The grant id of an attempt that runs polled: no interrupt was bound.
pub const POLLED: u64 = 0;

pub fn bind(dev: Found, claim_epoch: u64, mmio: &MmioMapOut) -> Result<IrqBindOut, &'static str> {
    let mut out = IrqBindOut { grant_id: 0, vector: 0 };
    if intx_usable(dev.irq_pin, dev.irq_line) {
        let r = mk_irq_bind(dev.device_id, claim_epoch, dev.irq_line as u32, 0, 0, &mut out);
        if r >= 0 {
            return Ok(out);
        }
        if after_intx(r) == Refusal::ClaimLost {
            return Err(give_back(dev, mmio));
        }
    }
    let r = mk_irq_bind(dev.device_id, claim_epoch, 0, MK_IRQ_BIND_MSIX, 1, &mut out);
    if r >= 0 {
        return Ok(out);
    }
    if after_msix(r) == Refusal::ClaimLost {
        return Err(give_back(dev, mmio));
    }
    Ok(IrqBindOut { grant_id: POLLED, vector: 0 })
}

// The claim is gone: release the register mapping and the claim.
fn give_back(dev: Found, mmio: &MmioMapOut) -> &'static str {
    let _ = mk_mmio_unmap(mmio.grant_id);
    let _ = mk_device_release(dev.device_id);
    "iwlwifi: claim lost during irq bind"
}
