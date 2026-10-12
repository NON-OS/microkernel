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

//! The PHY's register access through MDIC, the IEEE control register, the
//! I210/I211 page register and the locks shared with firmware. MDIC, SWSM
//! and SW_FW_SYNC: observed in Linux igb, unverified against Intel's
//! datasheets. The control register is IEEE 802.3 clause 22's register 0.

pub const MDIC_DATA_MASK: u32 = 0xFFFF;
pub const MDIC_REG_SHIFT: u32 = 16;
pub const MDIC_PHY_SHIFT: u32 = 21;
pub const MDIC_OP_WRITE: u32 = 1 << 26;
pub const MDIC_OP_READ: u32 = 1 << 27;
pub const MDIC_READY: u32 = 1 << 28;
pub const MDIC_ERROR: u32 = 1 << 30;
/// Clause 22 has five register address bits.
pub const MAX_PHY_REG: u32 = 0x1F;
/// The internal copper PHY answers at address 1 (observed in Linux igb and
/// in QEMU's igb model, which answers nowhere else).
pub const PHY_ADDR: u32 = 1;

/// Register 0, control, and its restart, power down and autonegotiation
/// enable bits.
pub const PHY_CONTROL: u32 = 0x00;
pub const MII_CR_RESTART_AUTO_NEG: u16 = 0x0200;
pub const MII_CR_POWER_DOWN: u16 = 0x0800;
pub const MII_CR_AUTO_NEG_EN: u16 = 0x1000;
/// The I210/I211 PHY's page register (observed in Linux igb, unverified).
pub const GS40G_PAGE_SELECT: u32 = 0x16;

/// SWSM: the software semaphore, then the software/firmware one.
pub const SWSM_SMBI: u32 = 1 << 0;
pub const SWSM_SWESMBI: u32 = 1 << 1;

/// SW_FW_SYNC's PHY bits by port; firmware's copy of each sits 16 up.
pub const SWFW_PHY_SM: [u32; 4] = [0x02, 0x04, 0x20, 0x40];
pub const SWFW_FW_SHIFT: u32 = 16;
