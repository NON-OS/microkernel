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
    /// The kind of controller on the bus this driver serves. A card reader
    /// has none, an empty reader being no missing disk; nor has a virtual
    /// adapter, whose bus the listing does not name.
    pub fn controller(self) -> Option<Controller> {
        match self {
            Driver::VirtioBlk => Some(Controller::VirtioBlk),
            Driver::Nvme => Some(Controller::Nvme),
            Driver::Ahci => Some(Controller::Ahci),
            Driver::Wire => Some(Controller::Emmc),
            Driver::Ufs => Some(Controller::Ufs),
            Driver::Rtsx | Driver::Ide | Driver::Scsi => None,
        }
    }

    /// Whether this driver serves a disk from controller `c`.
    pub fn serves(self, c: Controller) -> bool {
        self.controller() == Some(c)
    }

    /// The bus, as a person sees it in the disk list.
    pub fn label(self) -> &'static [u8] {
        match self {
            Driver::VirtioBlk => b"virtio",
            Driver::Nvme => b"NVMe",
            Driver::Ahci => b"SATA",
            Driver::Wire => b"eMMC",
            Driver::Ufs => b"UFS",
            Driver::Rtsx => b"SD",
            Driver::Ide => b"IDE",
            Driver::Scsi => b"SCSI",
        }
    }
}
