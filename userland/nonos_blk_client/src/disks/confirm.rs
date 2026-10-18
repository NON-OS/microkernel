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

//! The word a person types to erase a disk.

use alloc::string::String;

use super::scan::Disk;
use super::word::serial_word;
use crate::driver::Driver;

impl Disk {
    /// The word a person types to erase this disk: the last four characters
    /// of its serial when the part has one, so the word is on the sticker
    /// and on no other disk in the machine; the bus name otherwise, with
    /// the instance after it past the first, so two SATA disks are told
    /// apart.
    pub fn confirm_word(&self) -> String {
        if let Some(word) = self.identity.as_ref().and_then(|id| serial_word(id.serial_str())) {
            return word;
        }
        let bus = match self.driver {
            Driver::Nvme => "nvme",
            Driver::Ahci => "sata",
            Driver::VirtioBlk => "virtio",
            Driver::Rtsx => "sd",
        };
        match self.instance {
            0 => String::from(bus),
            n => alloc::format!("{bus}{n}"),
        }
    }
}
