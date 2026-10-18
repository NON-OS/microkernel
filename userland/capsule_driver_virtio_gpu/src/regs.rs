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

//! The three capability regions the driver reaches, each a bounds-checked
//! view of its MMIO grant (`nonos_virtio::Mmio`): a read past a region
//! answers all ones and a write past it is dropped, so no offset worked out
//! from what the device said can leave it. The legacy window and its port
//! I/O are gone with the legacy transport, which no virtio-gpu presents.

use nonos_virtio::Mmio;

#[derive(Clone, Copy)]
pub struct Regs {
    /// struct virtio_pci_common_cfg.
    pub common: Mmio,
    pub notify: Mmio,
    /// The control queue's doorbell: an offset into `notify` that the shared
    /// notify arithmetic checked to fall inside what is mapped.
    pub doorbell: usize,
    /// struct virtio_gpu_config.
    pub device: Mmio,
}

impl Regs {
    /// Tell the device queue `queue` has a command.
    pub fn notify(self, queue: u16) {
        self.notify.w16(self.doorbell, queue);
    }

    /// One field of struct virtio_gpu_config.
    pub fn config_r32(self, offset: usize) -> u32 {
        self.device.r32(offset)
    }
}
