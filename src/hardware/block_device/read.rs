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
    let copy_answers = super::kept::chosen().is_none_or(|b| b.disk.copy_answers);
    if copy_answers && super::mirror::read(lba, out) {
        return Ok(());
    }
    read_on(selected()?, lba, out)
}

/// A read from one named backend, for the probe that picks the backend.
pub(super) fn read_on(b: Backend, lba: u64, out: &mut [u8]) -> Result<(), BlockDeviceError> {
    (b.disk.read)(b.key, lba, out)
}

/// The first eight bytes of the sector at `lba` on `b`, or the read's error.
pub(super) fn head(b: Backend, lba: u64) -> Result<[u8; 8], BlockDeviceError> {
    let mut sector = [0u8; 512];
    read_on(b, lba, &mut sector)?;
    let mut head = [0u8; 8];
    head.copy_from_slice(&sector[..8]);
    Ok(head)
}
