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

//! The legacy handshake (recalled): reset, acknowledge, driver, no features,
//! queue 0 at the device's own size, then DRIVER_OK. FEATURES_OK is not a
//! legacy status bit and is not written.

use super::core::VirtioRngDevice;
use super::regs::*;
use crate::sys::io;

/// Reset the device, say a driver is here, and read queue 0's size.
pub(super) fn negotiate(iobase: u16) -> Result<u16, &'static str> {
    // SAFETY: every port below is a register of the device's own I/O BAR.
    unsafe {
        io::outb(iobase + STATUS, 0);
        io::outb(iobase + STATUS, STATUS_ACKNOWLEDGE);
        io::outb(iobase + STATUS, STATUS_ACKNOWLEDGE | STATUS_DRIVER);
        let _offered = io::inl(iobase + HOST_FEATURES);
        io::outl(iobase + GUEST_FEATURES, 0);
        io::outw(iobase + QUEUE_SEL, 0);
        match io::inw(iobase + QUEUE_NUM) {
            0 => Err("virtio-rng: queue 0 not available"),
            size => Ok(size),
        }
    }
}

impl VirtioRngDevice {
    /// Give the device its ring and let it run.
    pub(super) fn start(&mut self) -> Result<(), &'static str> {
        let pfn = self.queue.pfn().ok_or("virtio-rng: ring above the 32-bit PFN")?;
        self.write16(QUEUE_SEL, 0);
        self.write32(QUEUE_PFN, pfn);
        let s = self.read8(STATUS);
        self.write8(STATUS, s | STATUS_DRIVER_OK);
        if self.read8(STATUS) & STATUS_DRIVER_OK == 0 {
            return Err("virtio-rng: device refused DRIVER_OK");
        }
        Ok(())
    }
}
