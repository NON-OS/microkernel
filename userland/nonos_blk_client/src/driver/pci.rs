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

//! What a storage controller on the PCI bus is, from its ids alone.
//!
//! A driver that gave up exits and its service name goes with it, so a
//! disk behind it would leave no trace in the list. The controller is
//! still on the bus, and naming it is how the list says a disk is there
//! that nothing is serving. Intel's RST and VMD modes hide the NVMe and
//! SATA disks behind a RAID controller no NONOS driver speaks to; the
//! remedy is a firmware setting, and the person can only make it when
//! they are told.

/// The kinds of storage controller the installer has something to say about.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Controller {
    Nvme,
    Ahci,
    VirtioBlk,
    /// Intel RST in RAID mode, or a VMD domain the disks sit behind.
    IntelRaid,
    /// An Intel eMMC host: the soldered disk of an Atom-class laptop,
    /// served by driver.sdhci0 on the block wire.
    Emmc,
    /// A UFS host (class 01h/09h), served by driver.ufs0 on the block wire.
    Ufs,
}

const CLASS_STORAGE: u8 = 0x01;
const SUBCLASS_RAID: u8 = 0x04;
const SUBCLASS_SATA: u8 = 0x06;
const SUBCLASS_NVM: u8 = 0x08;
const SUBCLASS_UFS: u8 = 0x09;
const PROGIF_NVME: u8 = 0x02;

const VENDOR_INTEL: u16 = 0x8086;
const VENDOR_VIRTIO: u16 = 0x1AF4;
/* Legacy and modern virtio-blk. */
const VIRTIO_BLK: [u16; 2] = [0x1001, 0x1042];

const CLASS_SYSTEM: u8 = 0x08;
const SUBCLASS_SDHCI: u8 = 0x05;

// The Intel ids by name, beside this file so a proof that mounts it by
// path takes them too.
#[path = "pci_ids.rs"]
mod ids;
pub use ids::{INTEL_EMMC, INTEL_SATA_RAID, INTEL_VMD};

/// The controller a PCI function is, or `None` for one the disk list has
/// nothing to say about.
pub fn classify(
    vendor: u16,
    device: u16,
    class: u8,
    subclass: u8,
    progif: u8,
) -> Option<Controller> {
    let intel = vendor == VENDOR_INTEL;
    if intel && (INTEL_VMD.contains(&device) || INTEL_SATA_RAID.contains(&device)) {
        return Some(Controller::IntelRaid);
    }
    if intel && class == CLASS_SYSTEM && subclass == SUBCLASS_SDHCI && INTEL_EMMC.contains(&device)
    {
        return Some(Controller::Emmc);
    }
    if class != CLASS_STORAGE {
        return None;
    }
    match (subclass, progif) {
        (SUBCLASS_NVM, PROGIF_NVME) => Some(Controller::Nvme),
        /* The SATA driver takes every interface value: some vendors report
         * 00h for an AHCI HBA. */
        (SUBCLASS_SATA, _) => Some(Controller::Ahci),
        (SUBCLASS_UFS, _) => Some(Controller::Ufs),
        (SUBCLASS_RAID, _) if intel => Some(Controller::IntelRaid),
        _ if vendor == VENDOR_VIRTIO && VIRTIO_BLK.contains(&device) => Some(Controller::VirtioBlk),
        _ => None,
    }
}
