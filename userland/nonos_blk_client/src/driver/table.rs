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
    pub fn service(self) -> &'static [u8] {
        match self {
            Driver::VirtioBlk => b"driver.virtio_blk0",
            Driver::Nvme => b"driver.nvme0",
            Driver::Ahci => b"driver.ahci0",
        }
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

    /// What a person sees in the disk list.
    pub fn label(self) -> &'static [u8] {
        match self {
            Driver::VirtioBlk => b"virtio-blk",
            Driver::Nvme => b"NVMe",
            Driver::Ahci => b"SATA (AHCI)",
        }
    }
}
