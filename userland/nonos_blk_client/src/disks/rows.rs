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

//! The rows a survey is made of: a disk to install to, a disk withheld, and
//! a driver that gave no disk.

use alloc::string::String;

use super::contents::Contents;
use super::fault::fault;
use super::survey::Disk;
use crate::device::BlockDevice;
use crate::driver::Driver;
use crate::error::BlkError;

pub fn target(device: BlockDevice, contents: Contents) -> Disk {
    Disk {
        driver: device.driver,
        instance: device.instance,
        unit: device.unit,
        device: Some(device),
        size: device.bytes(),
        block: device.geometry.lba_size(),
        contents,
        identity: device.identity().ok().flatten(),
        fault: None,
    }
}

/// A disk that may not be installed to, and `why`.
pub fn withheld(device: BlockDevice, why: &str) -> Disk {
    Disk {
        driver: device.driver,
        instance: device.instance,
        unit: device.unit,
        device: None,
        size: device.bytes(),
        block: device.geometry.lba_size(),
        contents: Contents::probe(&device),
        identity: device.identity().ok().flatten(),
        fault: Some(String::from(why)),
    }
}

pub fn failed(driver: Driver, instance: u8, unit: u16, e: &BlkError) -> Disk {
    Disk {
        driver,
        instance,
        unit,
        device: None,
        size: 0,
        block: 0,
        contents: Contents::Unknown,
        identity: None,
        fault: Some(fault(driver, instance, e)),
    }
}
