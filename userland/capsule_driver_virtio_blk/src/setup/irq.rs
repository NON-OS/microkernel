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

//! The interrupt: one MSI-X vector of the device's own when the function
//! has MSI-X, else its INTx line. INTx was asked for first, a shared,
//! level-triggered line (the source of the rearm and ISR-read workarounds
//! in `io::rearm`) chosen over a dedicated vector; and a function with no
//! routed line was never found at all, so MSI-X was never reached for it.

use nonos_libc::{mk_irq_bind, IrqBindOut, MK_IRQ_BIND_MSIX};

use crate::discover::Found;

pub struct Bound {
    pub grant: u64,
    /// MSI-X, so the legacy device config sits after the vector registers
    /// and the queue must be pointed at table entry 0.
    pub msix: bool,
}

pub fn bind(dev: &Found, claim_epoch: u64) -> Result<Bound, &'static str> {
    let mut out = IrqBindOut { grant_id: 0, vector: 0 };
    if mk_irq_bind(dev.device_id, claim_epoch, 0, MK_IRQ_BIND_MSIX, 1, &mut out) >= 0 {
        return Ok(Bound { grant: out.grant_id, msix: true });
    }
    let line = dev.intx.ok_or("virtio-blk: no MSI-X vector and no routed INTx line")?;
    if mk_irq_bind(dev.device_id, claim_epoch, u32::from(line), 0, 0, &mut out) < 0 {
        return Err("virtio-blk: irq bind failed");
    }
    Ok(Bound { grant: out.grant_id, msix: false })
}
