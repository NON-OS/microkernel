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

use crate::discover::Found;

/// Bind the controller's interrupt: on PCI, MSI-X, or MSI on a controller
/// without it, as Intel PCH controllers are, which otherwise polled every
/// wait; a device-tree controller's GIC SPI as the broker published it. The
/// SPI is level triggered, and `ack_irq` clears IMAN.IP before acking, so the
/// line has dropped by the time it is unmasked. Best-effort: the driver still
/// decides every command and transfer from the event ring and only uses the
/// interrupt to park between polls, so a device with none, or a bind that
/// fails on real firmware, still works over polling. A zero grant means no
/// interrupt was bound.
pub fn irq_bind(dev: Found, claim_epoch: u64) -> IrqBindOut {
    if !dev.on_pci {
        let mut out = IrqBindOut { grant_id: 0, vector: 0 };
        if dev.spi != 0 && mk_irq_bind(dev.device_id, claim_epoch, dev.spi, 0, 0, &mut out) >= 0 {
            return out;
        }
        return IrqBindOut { grant_id: 0, vector: 0 };
    }
    for kind in [MK_IRQ_BIND_MSIX, MK_IRQ_BIND_MSI] {
        let mut out = IrqBindOut { grant_id: 0, vector: 0 };
        if mk_irq_bind(dev.device_id, claim_epoch, 0, kind, 1, &mut out) >= 0 {
            return out;
        }
    }
    IrqBindOut { grant_id: 0, vector: 0 }
}
