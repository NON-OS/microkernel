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

use crate::drivers::pci::PciAddress;
use crate::drivers::virtio_rng::queue::RngQueue;

use super::regs::STATUS;

pub(in crate::drivers::virtio_rng) struct VirtioRngDevice {
    pub(super) iobase: u16,
    pub(super) address: PciAddress,
    pub(super) queue: RngQueue,
    /// Regions this driver handed the device with no IOMMU domain confining
    /// them: the virtqueue and the data buffer, once the device accepted them.
    unconfined: u32,
}

/// The virtqueue and the data buffer.
const DMA_REGIONS: u32 = 2;

impl VirtioRngDevice {
    pub(in crate::drivers::virtio_rng) fn probe(
        iobase: u16,
        address: PciAddress,
    ) -> Result<Self, &'static str> {
        let size = super::init::negotiate(iobase)?;
        let queue = RngQueue::new(size, iobase + super::regs::QUEUE_NOTIFY)?;
        let mut dev = Self { iobase, address, queue, unconfined: 0 };
        dev.start()?;
        dev.unconfined = DMA_REGIONS;
        crate::memory::iommu::note_unconfined(DMA_REGIONS);
        Ok(dev)
    }

    pub(in crate::drivers::virtio_rng) fn at(&self, address: PciAddress) -> bool {
        self.address == address
    }
}

impl Drop for VirtioRngDevice {
    /// Reset first, so the device has stopped writing before the queue's
    /// frames, dropped after this, go back to the allocator.
    fn drop(&mut self) {
        self.write8(STATUS, 0);
        let mut tries = 1000;
        while self.read8(STATUS) != 0 && tries > 0 {
            core::hint::spin_loop();
            tries -= 1;
        }
        crate::memory::iommu::note_unconfined_released(self.unconfined);
    }
}
