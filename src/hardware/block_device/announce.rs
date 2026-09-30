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

//! Saying, once, which disk the block layer settled on.

use super::backend::Backend;

pub(super) fn announce(backend: Backend) -> Backend {
    let line = match backend {
        Backend::Nvme => "[BLOCK] NONOS disk on NVMe (driver.nvme0)",
        Backend::Ahci => "[BLOCK] NONOS disk on SATA (driver.ahci0)",
        Backend::VirtioBlk => "[BLOCK] NONOS disk on virtio-blk (driver.virtio_blk0)",
        Backend::UsbMsc => "[BLOCK] NONOS disk on USB mass storage (driver.usb_msc0)",
    };
    crate::sys::serial::println(line.as_bytes());
    crate::log::info!("{}", line);
    backend
}
