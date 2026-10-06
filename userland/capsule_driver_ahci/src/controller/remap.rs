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

//! Intel RST "RAID On" can hide NVMe drives behind the SATA controller's
//! ABAR: the NVMe function vanishes from PCI and only the RST driver reaches
//! it. Linux cannot use such a drive either; ahci_remap_check counts them
//! and tells the owner to switch the firmware to AHCI mode. These are its
//! registers (include/linux/ahci-remap.h) and its rule, pure for the proofs.

/// Vendor-specific capabilities; bit 0 says remapping may be present.
pub const VSCAP: u32 = 0xa4;
/// One bit per remap slot that is in use.
pub const REMAP_CAP: u32 = 0x800;
/// The class code of the device in remap slot 0; slot i is 0x80 further.
pub const REMAP_DCC: u32 = 0x880;
pub const REMAP_DCC_STRIDE: u32 = 0x80;
pub const MAX_REMAP: usize = 3;
/// PCI class 01h subclass 08h prog-if 02h: an NVMe controller.
const CLASS_NVME: u32 = 0x01_08_02;
/// Linux looks only at an ABAR of 512 KiB or more, which RST in RAID mode
/// gives the controller so the remapped devices' registers fit behind it.
const MIN_REMAP_ABAR: u64 = 512 * 1024;

/// Whether this ABAR may carry remapped devices, and enough of it is mapped
/// to read every slot's class code.
pub const fn may_remap(abar_size: u64, mapped: u64) -> bool {
    let last = REMAP_DCC + (MAX_REMAP as u32 - 1) * REMAP_DCC_STRIDE + 4;
    abar_size >= MIN_REMAP_ABAR && mapped >= last as u64
}

/// How many NVMe drives RST has remapped, from VSCAP, REMAP_CAP and each
/// slot's class code.
pub fn remapped_nvme(vscap: u32, remap_cap: u32, dcc: [u32; MAX_REMAP]) -> u32 {
    if vscap & 1 == 0 {
        return 0;
    }
    let mut n = 0;
    for (i, class) in dcc.iter().enumerate() {
        if remap_cap & (1 << i) != 0 && *class == CLASS_NVME {
            n += 1;
        }
    }
    n
}
