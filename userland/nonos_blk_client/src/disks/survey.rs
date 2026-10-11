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

//! What a look at the machine's disks found. A driver that did not answer
//! is a row of its own rather than dropped, so the list never looks
//! complete when it is not.

use alloc::string::String;
use alloc::vec::Vec;

use super::contents::Contents;
use crate::device::{BlockDevice, Identity};
use crate::driver::Driver;

pub struct Disk {
    pub driver: Driver,
    /// Which of the driver's service names serves it.
    pub instance: u8,
    /// The device to install to. `None` for every row that is not a
    /// target: a driver that did not answer, a controller nothing serves,
    /// a disk that may be the one this boot came from.
    pub device: Option<BlockDevice>,
    /// Bytes, from the driver's capacity in the disk's own blocks; 0 when
    /// no driver said.
    pub size: u64,
    /// The disk's own block size in bytes; 0 when no driver said.
    pub block: u32,
    pub contents: Contents,
    pub identity: Option<Identity>,
    /// Why this row is not a target, when it is not.
    pub fault: Option<String>,
}

/// One look at the machine's disks.
pub struct Survey {
    pub disks: Vec<Disk>,
    /// An Intel RST or VMD controller is on the bus. With it on, the NVMe
    /// and SATA disks sit behind a RAID controller no NONOS driver speaks
    /// to, and the firmware's storage mode is what brings them back.
    pub raid: bool,
}

impl Survey {
    /// The rows that can be installed to.
    pub fn targets(&self) -> usize {
        self.disks.iter().filter(|d| d.device.is_some()).count()
    }

    /// Whether to tell the person to change the firmware's storage mode:
    /// only when the RAID controller is there and no disk is, since a disk
    /// found means the firmware already lets this system reach one.
    pub fn raid_hides_disks(&self) -> bool {
        self.raid && self.targets() == 0
    }

    /// Whether a later look may find more: there is nothing to install to,
    /// or a row says a driver is missing or did not answer.
    pub fn incomplete(&self) -> bool {
        self.targets() == 0 || self.disks.iter().any(Disk::missing)
    }
}
