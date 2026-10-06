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

use super::backend::Backend;
use super::map_ahci::map_ahci_error;
use super::map_nvme::map_nvme_error;
use super::map_usb_msc::map_usb_msc_error;
use super::map_virtio::map_virtio_error;
use super::select::selected;
use super::BlockDeviceError;

/// The kept disk's size; until one is kept, the size of the boot disk the
/// loader copied ranges of, as `read` answers from that copy.
pub fn capacity() -> Result<u64, BlockDeviceError> {
    if super::select::chosen().is_none() {
        if let Some(sectors) = super::mirror::capacity() {
            return Ok(sectors);
        }
    }
    capacity_on(selected()?)
}

/// The size of one named backend's disk, for the probe that picks the backend.
pub(super) fn capacity_on(backend: Backend) -> Result<u64, BlockDeviceError> {
    match backend {
        Backend::VirtioBlk => {
            crate::hardware::virtio_blk_capsule::capacity().map_err(map_virtio_error)
        }
        Backend::Ahci => crate::hardware::ahci_capsule::capacity().map_err(map_ahci_error),
        Backend::Nvme => crate::hardware::nvme_capsule::capacity().map_err(map_nvme_error),
        Backend::UsbMsc => crate::hardware::usb_msc_capsule::capacity().map_err(map_usb_msc_error),
    }
}
