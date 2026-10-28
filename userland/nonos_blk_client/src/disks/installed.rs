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

//! Whether this boot runs from an installed NØNOS: the disk the kernel's
//! block layer chose (D13b) is an internal one, not a USB stick, and its
//! disk plan names a data volume, which a live stick's plan never does.
//! Asked once, when the installer opens, for the row that prepares the
//! computer for an update; one read of one sector of one disk.

use nonos_disk::names_a_volume;

use super::chosen::ask;
use super::in_use::{AHCI, NVME, VIRTIO_BLK};
use crate::device::BlockDevice;
use crate::driver::Driver;
use crate::sink::DeviceSink;

pub fn running_installed() -> bool {
    let Some((code, instance)) = ask() else { return false };
    let driver = match code {
        VIRTIO_BLK => Driver::VirtioBlk,
        AHCI => Driver::Ahci,
        NVME => Driver::Nvme,
        /* A USB stick, or a backend this does not know: not an install. */
        _ => return false,
    };
    let Some(port) = BlockDevice::lookup(driver, instance) else { return false };
    let Ok(Some(device)) = BlockDevice::at(driver, instance, port, 0) else { return false };
    names_a_volume(&mut DeviceSink { device })
}
