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

//! A disk's first sectors read through its driver, what the list says
//! about them, and the row for a disk whose read failed: listed with the
//! driver's words and not offered.

use alloc::format;

use super::contents::Contents;
use super::scan::Disk;
use crate::device::BlockDevice;
use crate::status_text::describe;

impl Contents {
    pub fn probe(device: &BlockDevice) -> Contents {
        Contents::read_with(device.sectors, |head| device.read(0, head).map_err(|e| e.code()))
    }

    pub fn text(self) -> &'static str {
        match self {
            Contents::Blank => "blank",
            Contents::Nonos => "NONOS installed",
            Contents::OtherGpt => "another system (GPT)",
            Contents::Mbr => "another system (MBR)",
            Contents::Unknown => "unrecognised contents",
            Contents::Unread(_) => "did not answer a read",
        }
    }
}

/// "its first sectors did not read: the device did not answer in time".
pub(super) fn unread(device: BlockDevice, status: i32) -> Disk {
    Disk {
        driver: device.driver,
        instance: device.instance,
        device: None,
        size: device.bytes(),
        block: device.geometry.lba_size(),
        contents: Contents::Unread(status),
        identity: device.identity().ok().flatten(),
        fault: Some(format!("its first sectors did not read: {}; not offered", describe(status))),
    }
}
