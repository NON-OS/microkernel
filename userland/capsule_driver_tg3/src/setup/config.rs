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

//! Configuration-space access through the broker. Reads are any width;
//! writes are 16 bits wide, and the broker lets move only the bits its
//! quirk_bits rule names for a Broadcom Ethernet function.

use nonos_libc::{mk_pci_config_read, mk_pci_config_write};

use crate::constants::pcicfg::{PCI_CAPABILITY_LIST, PCI_CAP_ID_EXP};

#[derive(Clone, Copy)]
pub struct Config {
    pub device_id: u64,
    pub epoch: u64,
}

/// Capability pointers sit at 0x40 or above, four bytes apart, so a list
/// longer than this loops.
const MAX_CAPS: usize = 48;

impl Config {
    pub fn read(&self, offset: u32, width: u32) -> Result<u32, &'static str> {
        let r = mk_pci_config_read(self.device_id, self.epoch, offset, width);
        if r < 0 {
            return Err("config read refused");
        }
        Ok(r as u32)
    }

    pub fn write16(&self, offset: u32, value: u16) -> Result<(), &'static str> {
        if mk_pci_config_write(self.device_id, self.epoch, offset, value) < 0 {
            return Err("config write refused");
        }
        Ok(())
    }

    /// The PCI Express capability's offset. Every 57765-class part has one.
    pub fn pcie_capability(&self) -> Result<u32, &'static str> {
        let mut at = self.read(PCI_CAPABILITY_LIST, 1)? & 0xFC;
        for _ in 0..MAX_CAPS {
            if at < 0x40 {
                break;
            }
            let header = self.read(at, 2)?;
            if header as u8 == PCI_CAP_ID_EXP {
                return Ok(at);
            }
            at = (header >> 8) & 0xFC;
        }
        Err("no PCI Express capability")
    }
}
