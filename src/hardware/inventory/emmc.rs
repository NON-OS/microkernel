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

//! Intel eMMC hosts: the SD Host Controller on Atom, Celeron and Pentium
//! Silver platforms that the board's soldered eMMC hangs off, class 08h
//! subclass 05h. These are the ids Linux's sdhci-pci driver names eMMC
//! (drivers/mmc/host/sdhci-pci-core.c). The same platforms carry SD card
//! and SDIO hosts with the same class; they are not a disk and are not
//! listed. Until the eMMC driver gets its own publisher keys it lives in
//! the AHCI capsule, which is what a host here starts.
//!
//! Pure, with no imports, so host proofs can include it by path.

const INTEL: u16 = 0x8086;

/// Bay Trail (two), Braswell/Cherry Trail, Broxton, Broxton-M, Apollo Lake,
/// Gemini Lake, Cannon Point, Ice Point, Cedar Fork, Elkhart Lake, Jasper
/// Lake.
pub const INTEL_EMMC_DEVICE_IDS: [u16; 12] = [
    0x0f14, 0x0f50, 0x2294, 0x0acc, 0x1aa8, 0x5acc, 0x31cc, 0x9dc4, 0x34c4, 0x18db, 0x4b47, 0x4dc4,
];

const PCI_CLASS_SYSTEM: u8 = 0x08;
const PCI_SUBCLASS_SDHCI: u8 = 0x05;

/// Whether a function is an Intel eMMC host.
pub const fn is_intel_emmc(vendor: u16, device: u16, class: u8, subclass: u8) -> bool {
    if vendor != INTEL || class != PCI_CLASS_SYSTEM || subclass != PCI_SUBCLASS_SDHCI {
        return false;
    }
    let mut i = 0;
    while i < INTEL_EMMC_DEVICE_IDS.len() {
        if INTEL_EMMC_DEVICE_IDS[i] == device {
            return true;
        }
        i += 1;
    }
    false
}
