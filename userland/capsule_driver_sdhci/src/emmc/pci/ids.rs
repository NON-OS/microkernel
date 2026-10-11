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

//! The SDHCI class code and the Intel device ids that are and are not eMMC.

pub const PCI_CLASS_SYSTEM: u8 = 0x08;
pub const PCI_SUBCLASS_SDHCI: u8 = 0x05;
pub const PCI_VENDOR_INTEL: u16 = 0x8086;

/// Intel eMMC hosts: each is one embedded slot at BAR0, offset 0, wired 8
/// bits wide to the board's eMMC.
pub const INTEL_EMMC: &[u16] = &[
    0x0f14, // Bay Trail (BYT_EMMC)
    0x0f50, // Bay Trail (BYT_EMMC2)
    0x2294, // Braswell / Cherry Trail (BSW_EMMC)
    0x0acc, // Broxton (BXT_EMMC)
    0x1aa8, // Broxton-M (BXTM_EMMC)
    0x5acc, // Apollo Lake (APL_EMMC)
    0x31cc, // Gemini Lake (GLK_EMMC)
    0x9d2b, // Sunrise Point (SPT_EMMC)
    0x9dc4, // Cannon Point (CNP_EMMC)
    0x34c4, // Ice Point (ICP_EMMC)
    0x18db, // Cedar Fork (CDF_EMMC)
    0x4b47, // Elkhart Lake (EHL_EMMC)
    0x4dc4, // Jasper Lake (JSL_EMMC)
];

/// Intel SD card reader and SDIO hosts: never the internal disk.
pub const INTEL_NOT_EMMC: &[u16] = &[
    0x31ca, // GLK_SD
    0x31d0, // GLK_SDIO
    0x5aca, // APL_SD
    0x5ad0, // APL_SDIO
    0x0aca, // BXT_SD
    0x0ad0, // BXT_SDIO
    0x0f15, // BYT_SDIO
    0x0f16, // BYT_SD
    0x2295, // BSW_SDIO
    0x2296, // BSW_SD
    0x9d2c, // SPT_SDIO
    0x9d2d, // SPT_SD
    0x4b48, // EHL_SD
    0x4df8, // JSL_SD
];
