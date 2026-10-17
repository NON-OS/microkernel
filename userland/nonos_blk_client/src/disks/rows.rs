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

//! The rows of the disk list that are not targets, and why.

use alloc::string::String;

use super::contents::Contents;
use super::scan::Disk;
use crate::device::BlockDevice;
use crate::driver::Driver;
use crate::error::BlkError;

use super::fault::fault;

/// A disk that can be installed to.
pub(super) fn target(
    driver: Driver,
    instance: u8,
    device: BlockDevice,
    contents: Contents,
) -> Disk {
    Disk {
        driver,
        instance,
        device: Some(device),
        size: device.bytes(),
        block: device.geometry.lba_size(),
        contents,
        identity: device.identity().ok().flatten(),
        fault: None,
    }
}

/// A driver that did not answer for `instance`.
pub(super) fn failed(driver: Driver, instance: u8, e: &BlkError, emmc_only: bool) -> Disk {
    Disk {
        driver,
        instance,
        device: None,
        size: 0,
        block: 0,
        contents: Contents::Unknown,
        identity: None,
        fault: Some(fault(driver, instance, e, emmc_only)),
    }
}

pub(super) fn withheld(device: BlockDevice) -> Disk {
    Disk {
        driver: device.driver,
        instance: device.instance,
        device: None,
        size: device.bytes(),
        block: device.geometry.lba_size(),
        contents: Contents::probe(&device),
        identity: device.identity().ok().flatten(),
        fault: Some(String::from(
            "holds the loader this boot ran, so it may be the boot disk: not offered",
        )),
    }
}
