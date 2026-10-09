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

use nonos_libc::{mk_irq_bind, IrqBindOut, MK_IRQ_BIND_MSI, MK_IRQ_BIND_MSIX};

use super::irq_plan::intx_routed;
use super::mark::mark;
use crate::discover::Found;
use crate::error::HdaResult;

/// Message interrupts first. NONOS has no ACPI _PRT walk, so the INTx line
/// config space names may not be the pin the controller is wired to; Intel
/// HDA controllers offer MSI and no MSI-X, and Linux snd_hda_intel enables
/// MSI on them by default (azx_probe, enable_msi).
pub fn bind(dev: Found, claim_epoch: u64) -> HdaResult<IrqBindOut> {
    let mut out = IrqBindOut { grant_id: 0, vector: 0 };
    let id = dev.device_id;
    if mk_irq_bind(id, claim_epoch, 0, MK_IRQ_BIND_MSIX, 1, &mut out) >= 0 {
        mark("[HDA] irq msi-x\n");
        return Ok(out);
    }
    if mk_irq_bind(id, claim_epoch, 0, MK_IRQ_BIND_MSI, 1, &mut out) >= 0 {
        mark("[HDA] irq msi\n");
        return Ok(out);
    }
    if intx_routed(dev.irq_pin, dev.irq_line) {
        if mk_irq_bind(id, claim_epoch, dev.irq_line as u32, 0, 0, &mut out) >= 0 {
            mark("[HDA] irq intx\n");
            return Ok(out);
        }
    }
    mark("[HDA] irq-polled\n");
    Ok(IrqBindOut { grant_id: 0, vector: 0 })
}
