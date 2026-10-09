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

//! Per-driver constants, copied from each driver's `protocol` module. They
//! are the contract those capsules publish; a change there is a change
//! here, and the driver's own tests are what hold the two together until
//! the three protocols become one.

use super::pci::Controller;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Driver {
    VirtioBlk,
    Nvme,
    Ahci,
}

pub const ALL: [Driver; 3] = [Driver::Nvme, Driver::Ahci, Driver::VirtioBlk];

/// Opcodes in the order capacity, read, write, flush.
pub struct Ops {
    pub capacity: u16,
    pub read: u16,
    pub write: u16,
    pub flush: u16,
}

impl Driver {
    /// The service name of instance `n` of this driver, `None` past the
    /// last one known. Each driver serves one controller under its `0`
    /// name today; the next names are looked up as well, so a kernel that
    /// registers a second NVMe or SATA instance has its disks listed with
    /// no change here, and a name nobody registered costs one lookup.
    pub fn service(self, n: u8) -> Option<&'static [u8]> {
        let names: &[&'static [u8]] = match self {
            /* Written as a call site, not an array entry: the capability
             * audit reads this name as the evidence that a client of this
             * crate needs StoreWrite. */
            Driver::VirtioBlk => return (n == 0).then_some(&b"driver.virtio_blk0"[..]),
            Driver::Nvme => &[b"driver.nvme0", b"driver.nvme1", b"driver.nvme2", b"driver.nvme3"],
            Driver::Ahci => &[b"driver.ahci0", b"driver.ahci1", b"driver.ahci2", b"driver.ahci3"],
        };
        names.get(n as usize).copied()
    }

    /// How many instance names [`Driver::service`] knows.
    pub fn instances(self) -> u8 {
        match self {
            Driver::VirtioBlk => 1,
            Driver::Nvme | Driver::Ahci => 4,
        }
    }

    /// The kind of controller on the bus this driver serves.
    pub fn controller(self) -> Controller {
        match self {
            Driver::VirtioBlk => Controller::VirtioBlk,
            Driver::Nvme => Controller::Nvme,
            Driver::Ahci => Controller::Ahci,
        }
    }

    /// Whether this driver serves a disk from controller `c`: the SATA
    /// driver also serves the eMMC hosts.
    pub fn serves(self, c: Controller) -> bool {
        c == self.controller() || (self == Driver::Ahci && c == Controller::Emmc)
    }

    pub fn magic(self) -> u32 {
        match self {
            Driver::VirtioBlk => 0x4E42_4C4B,
            Driver::Nvme => 0x4E4E_564D,
            Driver::Ahci => 0x4E41_4843,
        }
    }

    pub fn ops(self) -> Ops {
        match self {
            Driver::VirtioBlk => Ops { capacity: 2, read: 3, write: 4, flush: 5 },
            Driver::Nvme => Ops { capacity: 6, read: 7, write: 8, flush: 9 },
            Driver::Ahci => Ops { capacity: 4, read: 5, write: 6, flush: 7 },
        }
    }

    /// The bus, as a person sees it in the disk list.
    pub fn label(self) -> &'static [u8] {
        match self {
            Driver::VirtioBlk => b"virtio",
            Driver::Nvme => b"NVMe",
            Driver::Ahci => b"SATA",
        }
    }
}
