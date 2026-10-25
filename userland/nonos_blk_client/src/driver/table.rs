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
//! here, and blk_client_proofs, the drivers' own tests and
//! scripts/check_blk_wire.py hold the two together until the protocols
//! become one. Magics and opcodes are in `dialect.rs`, the controllers each
//! serves in `bus.rs`.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Driver {
    VirtioBlk,
    Nvme,
    Ahci,
    /// A driver on the block wire (driver.sdhci0, eMMC), speaking GEOMETRY
    /// where the others answer a capacity, and IDENTIFY.
    Wire,
    /// The UFS host's driver (driver.ufs0), on the same block wire.
    Ufs,
    /// The Realtek PCIe SD card reader, serving the card in its slot.
    Rtsx,
    /// The PIIX IDE controller's driver (driver.piix_ide0), on the block wire.
    Ide,
    /// The virtual SCSI adapters' drivers, on the block wire: virtio-scsi,
    /// VMware PVSCSI, BusLogic, LSI MPT and Hyper-V's synthetic SCSI.
    Scsi,
}

pub const ALL: [Driver; 8] = [
    Driver::Nvme,
    Driver::Ahci,
    Driver::Wire,
    Driver::Ufs,
    Driver::Ide,
    Driver::Scsi,
    Driver::VirtioBlk,
    Driver::Rtsx,
];
