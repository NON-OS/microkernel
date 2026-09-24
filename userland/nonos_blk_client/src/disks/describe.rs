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

//! How a disk describes itself to a person: its bus name, its size, and
//! the word that confirms erasing it.

use alloc::string::String;

use super::scan::Disk;
use crate::driver::Driver;

impl Disk {
    pub fn label(&self) -> &'static str {
        core::str::from_utf8(self.driver.label()).unwrap_or("disk")
    }

    pub fn bytes(&self) -> u64 {
        self.device.map(|d| d.bytes()).unwrap_or(0)
    }

    /// The word a person types to erase this disk: the last four characters
    /// of its serial when the part has one, so the word is on the sticker
    /// and on no other disk in the machine; the bus name otherwise.
    pub fn confirm_word(&self) -> String {
        if let Some(id) = &self.identity {
            let s = id.serial_str();
            if s.len() >= 4 {
                return s[s.len() - 4..].to_ascii_lowercase();
            }
        }
        String::from(match self.driver {
            Driver::Nvme => "nvme",
            Driver::Ahci => "sata",
            Driver::VirtioBlk => "virtio",
        })
    }
}
