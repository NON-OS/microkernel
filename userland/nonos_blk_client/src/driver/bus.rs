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

//! Which controllers on the bus a driver serves, and the bus as a person
//! sees it in the disk list.

use super::pci::Controller;
use super::table::Driver;

impl Driver {
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

    /// The bus, as a person sees it in the disk list.
    pub fn label(self) -> &'static [u8] {
        match self {
            Driver::VirtioBlk => b"virtio",
            Driver::Nvme => b"NVMe",
            Driver::Ahci => b"SATA",
        }
    }
}
