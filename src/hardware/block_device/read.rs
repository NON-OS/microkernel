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

/*
 * The loader's copy of the boot disk answers what it holds (the live plan
 * and the model files it names) until a disk is kept, so a live boot whose
 * stick no driver of the kernel's drives still opens its volume and imports
 * its model. It goes on answering once the kept disk is a USB stick: the
 * copy exists only for a live stick the loader came from, it holds the same
 * bytes, and a model import streamed through the stick driver can outlast
 * the kernel's wait on a slow stick. A kept NVMe, SATA or virtio disk
 * answers everything itself.
 */
pub fn read(lba: u64, out: &mut [u8]) -> Result<(), BlockDeviceError> {
    let copy_answers = matches!(super::select::chosen(), None | Some(Backend::UsbMsc));
    if copy_answers && super::mirror::read(lba, out) {
        return Ok(());
    }
    read_on(selected()?, lba, out)
}

/// A read from one named backend, for the probe that picks the backend.
pub(super) fn read_on(backend: Backend, lba: u64, out: &mut [u8]) -> Result<(), BlockDeviceError> {
    match backend {
        Backend::VirtioBlk => {
            crate::hardware::virtio_blk_capsule::read_blocks(lba, out).map_err(map_virtio_error)
        }
        Backend::Ahci => {
            crate::hardware::ahci_capsule::read_blocks(lba, out).map_err(map_ahci_error)
        }
        Backend::Nvme => {
            crate::hardware::nvme_capsule::read_blocks(lba, out).map_err(map_nvme_error)
        }
        Backend::UsbMsc => {
            crate::hardware::usb_msc_capsule::read_blocks(lba, out).map_err(map_usb_msc_error)
        }
    }
}
