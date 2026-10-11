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

//! The clock and power management unit and the PCI Express block window,
//! as tg3_reset_hw touches them for the 57765 class (tg3.h).

pub const TG3_CPMU_LSPD_10MB_CLK: usize = 0x3604;
pub const CPMU_LSPD_10MB_MACCLK_MASK: u32 = 0x001F_0000;
pub const CPMU_LSPD_10MB_MACCLK_6_25: u32 = 0x0013_0000;
pub const TG3_CPMU_PADRNG_CTL: usize = 0x3668;
pub const TG3_CPMU_PADRNG_CTL_RDIV2: u32 = 0x0004_0000;

/// The PCI Express TL/DL/PL register window, chosen by GRC_MODE's port bits.
pub const TG3_PCIE_TLDLPL_PORT: usize = 0x7C00;
pub const TG3_PCIE_PL_LO_PHYCTL5: usize = 0x14;
pub const TG3_PCIE_PL_LO_PHYCTL5_DIS_L2CLKREQ: u32 = 0x8000_0000;
pub const TG3_PCIE_DL_LO_FTSMAX: usize = 0x0C;
pub const TG3_PCIE_DL_LO_FTSMAX_MSK: u32 = 0x0000_00FF;
pub const TG3_PCIE_DL_LO_FTSMAX_VAL: u32 = 0x0000_002C;
