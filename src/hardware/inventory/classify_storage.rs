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

use super::family::HardwareFamily;
use super::vmd::is_intel_vmd;

const VENDOR_INTEL: u16 = 0x8086;
const VENDOR_VIRTIO: u16 = 0x1af4;
const SUBCLASS_RAID: u8 = 0x04;
const SUBCLASS_SATA: u8 = 0x06;
const SUBCLASS_NVM: u8 = 0x08;

/*
 * An Intel RST SATA controller in "RAID On" mode reports subclass 04h (RAID)
 * but is a standard AHCI HBA with its ABAR in BAR5; Linux's ahci driver binds
 * those by device id. Every Intel 01h/04h function that is not a VMD is taken
 * as one here, and `classify::classify_device` drops it again when BAR5 is no
 * memory BAR. A VMD is a family of its own, which nothing spawns for.
 */
pub(super) fn classify_storage(subclass: u8, vendor: u16, device: u16) -> HardwareFamily {
    if is_intel_vmd(vendor, device) {
        return HardwareFamily::StorageVmd;
    }
    if vendor == VENDOR_VIRTIO {
        return HardwareFamily::StorageVirtioBlk;
    }
    match subclass {
        SUBCLASS_SATA => HardwareFamily::StorageAhci,
        SUBCLASS_RAID if vendor == VENDOR_INTEL => HardwareFamily::StorageAhci,
        SUBCLASS_NVM => HardwareFamily::StorageNvme,
        _ => HardwareFamily::Unknown,
    }
}
