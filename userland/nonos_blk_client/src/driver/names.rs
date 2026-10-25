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


//! The service names each driver's instances register.

use super::table::Driver;

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
            Driver::Wire => &[b"driver.sdhci0"],
            Driver::Ufs => &[b"driver.ufs0"],
            Driver::Rtsx => &[b"driver.rtsx0"],
            Driver::Ide => &[b"driver.piix_ide0"],
            Driver::Scsi => &[
                b"driver.virtio_scsi0",
                b"driver.pvscsi0",
                b"driver.buslogic0",
                b"driver.mpt0",
                b"driver.storvsc0",
            ],
        };
        names.get(n as usize).copied()
    }

    /// How many instance names [`Driver::service`] knows.
    pub fn instances(self) -> u8 {
        match self {
            Driver::VirtioBlk | Driver::Wire | Driver::Ufs | Driver::Rtsx | Driver::Ide => 1,
            Driver::Nvme | Driver::Ahci => 4,
            Driver::Scsi => 5,
        }
    }
}
