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

//! How a disk describes itself to a person: its bus name, its size, the
//! word that confirms erasing it, and why it cannot be installed to when
//! it cannot.

use alloc::string::String;

use super::scan::Disk;
use super::word::serial_word;
use crate::driver::Driver;
use crate::wire::SECTOR_SIZE;

impl Disk {
    pub fn label(&self) -> &'static str {
        if self.identity.is_some_and(|id| id.emmc) {
            return "eMMC";
        }
        core::str::from_utf8(self.driver.label()).unwrap_or("disk")
    }

    /// The size in bytes, from the capacity in the disk's own blocks.
    pub fn bytes(&self) -> u64 {
        self.size
    }

    /// A row for a driver or a controller that gave no disk at all: what a
    /// later look may change, as a driver comes up late.
    pub fn missing(&self) -> bool {
        self.device.is_none() && self.size == 0
    }

    /// Why NONOS cannot go onto this disk although its driver serves it.
    /// Every layout the writer makes is in 512-byte blocks, and firmware
    /// reads a GPT in the disk's own: on a disk with 4096-byte blocks it
    /// looks for the table header at byte 4096, where the writer put none,
    /// and the disk would never boot.
    pub fn refusal(&self) -> Option<String> {
        let block = self.device.map(|d| d.geometry.lba_size())?;
        (block != SECTOR_SIZE as u32).then(|| {
            alloc::format!(
                "this disk uses {block}-byte blocks, and NONOS lays its partition table out in 512-byte ones, which firmware would not find here"
            )
        })
    }

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
        };
        match self.instance {
            0 => String::from(bus),
            n => alloc::format!("{bus}{n}"),
        }
    }
}
