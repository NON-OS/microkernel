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

//! What tells the device served from any other. The kernel chooses its disk
//! once a boot, by the NONOS store or disk plan on it, and never looks again
//! (src/hardware/block_device/select.rs), so once a device has been served,
//! one bound after it was let go is served only if it is the same: the
//! same unit's INQUIRY, size and block length, and USB serial number. Any
//! other was served before, and a store write landed on a user's other
//! stick. The port is not identity: a stick may come back on another. A
//! device with no serial number cannot be told from another of its make,
//! and is not served again (`release`).

use crate::disk::Disk;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Identity {
    /// The digests of what its INQUIRY said (None when it failed the
    /// command) and of its serial number (None when it has none), from
    /// disk/digest.rs.
    named: Option<u64>,
    serial: Option<u64>,
    lun: u8,
    blocks: u64,
    block_len: u32,
}

impl Identity {
    /// `disk`, sized, as its INQUIRY named it, with its serial number.
    pub fn of(disk: &Disk, named: Option<u64>, serial: Option<u64>) -> Self {
        let (lun, blocks, block_len) = (disk.lun, disk.blocks, disk.block_len);
        Self { named, serial, lun, blocks, block_len }
    }

    /// The same unit at the size `disk` has now: its medium taken afresh
    /// (`check`).
    pub fn sized(self, disk: &Disk) -> Self {
        Self { blocks: disk.blocks, block_len: disk.block_len, ..self }
    }

    /// Whether a serial number tells it from another device of its make.
    pub fn has_serial(&self) -> bool {
        self.serial.is_some()
    }

    /// Whether unit `lun` of the device with `serial` is this one, at
    /// whatever size: whose medium a change it says is about.
    pub fn is_unit(&self, serial: Option<u64>, lun: u8) -> bool {
        (self.serial, self.lun) == (serial, lun)
    }
}
