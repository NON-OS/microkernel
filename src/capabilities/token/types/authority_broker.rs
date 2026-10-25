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

use super::CapabilityToken;
use crate::capabilities::types::Capability;

/*
 * The five device checks take their own capability and nothing else
 *: Admin, which the installer holds for its reboot, no longer
 * drives a device, so an unclaimed device is reached only by a capsule
 * granted the bit for it.
 */
impl CapabilityToken {
    // Claim / release: with `DeviceEnum`, exclusive ownership of a device.
    #[inline]
    pub fn can_driver(&self) -> bool {
        self.grants(Capability::Driver)
    }
    // MMIO mapping, beyond `Driver`: a claim alone maps nothing.
    #[inline]
    pub fn can_mmio(&self) -> bool {
        self.grants(Capability::Mmio)
    }
    // IRQ binding, beyond `Driver`: interrupts from a claimed device.
    #[inline]
    pub fn can_irq(&self) -> bool {
        self.grants(Capability::Irq)
    }
    // DMA buffers, beyond `Driver`: memory the claimed device can reach.
    #[inline]
    pub fn can_dma(&self) -> bool {
        self.grants(Capability::Dma)
    }
    // PIO grants, beyond `Driver`: kernel-mediated `in`/`out` on a window.
    #[inline]
    pub fn can_pio(&self) -> bool {
        self.grants(Capability::Pio)
    }
    /*
     * The raw input ring carries every keystroke (K8). A dedicated input
     * driver posts into it under InputSource; only input_router, under
     * InputConsumer, drains or waits on it. Irq and Admin stand in for
     * neither: every device driver holds Irq, and a driver that could post
     * could forge keys, as one that could drain could read them.
     */
    #[inline]
    pub fn can_input_source(&self) -> bool {
        self.grants(Capability::InputSource)
    }
    #[inline]
    pub fn can_input_consumer(&self) -> bool {
        self.grants(Capability::InputConsumer)
    }
}
