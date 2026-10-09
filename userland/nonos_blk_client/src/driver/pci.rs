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
    /// An Intel eMMC host: the soldered disk of an Atom-class laptop. The
    /// SATA driver serves it until the eMMC driver has keys of its own.
    Emmc,
}

const CLASS_STORAGE: u8 = 0x01;
const SUBCLASS_RAID: u8 = 0x04;
const SUBCLASS_SATA: u8 = 0x06;
const SUBCLASS_NVM: u8 = 0x08;
const PROGIF_NVME: u8 = 0x02;

const VENDOR_INTEL: u16 = 0x8086;
const VENDOR_VIRTIO: u16 = 0x1AF4;
/* Legacy and modern virtio-blk. */
const VIRTIO_BLK: [u16; 2] = [0x1001, 0x1042];

/// Intel Volume Management Device functions. Some report the RAID
/// subclass, some a system peripheral class; the id names them either way.
pub const INTEL_VMD: [u16; 10] =
    [0x201D, 0x28C0, 0x467F, 0x4C3D, 0x9A0B, 0xA77F, 0x7D0B, 0xAD0B, 0xB06F, 0xB60B];

/// Intel eMMC hosts, as the kernel's inventory lists them
/// (src/hardware/inventory/emmc.rs); a proof holds the two lists together.
pub const INTEL_EMMC: [u16; 12] = [
    0x0F14, 0x0F50, 0x2294, 0x0ACC, 0x1AA8, 0x5ACC, 0x31CC, 0x9DC4, 0x34C4, 0x18DB, 0x4B47, 0x4DC4,
];
const CLASS_SYSTEM: u8 = 0x08;
const SUBCLASS_SDHCI: u8 = 0x05;

/// Intel SATA controllers in RAID (RST) mode, named by id as well as by
/// the RAID subclass they report.
pub const INTEL_SATA_RAID: [u16; 2] = [0x2822, 0x282A];

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
        (SUBCLASS_RAID, _) if intel => Some(Controller::IntelRaid),
        _ if vendor == VENDOR_VIRTIO && VIRTIO_BLK.contains(&device) => Some(Controller::VirtioBlk),
        _ => None,
    }
}
