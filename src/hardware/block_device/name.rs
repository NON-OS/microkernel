// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! What the probe log calls each backend's disk.

use super::backend::Backend;

pub(super) fn name(backend: Backend) -> &'static str {
    match backend {
        Backend::UsbMsc => "USB stick (driver.usb_msc0)",
        Backend::Nvme => "NVMe (driver.nvme0)",
        Backend::Ahci => "SATA (driver.ahci0)",
        Backend::VirtioBlk => "virtio-blk (driver.virtio_blk0)",
        Backend::Wire(i) => super::wire::shown(i),
    }
}
