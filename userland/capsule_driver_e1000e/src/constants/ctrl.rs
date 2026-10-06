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

//! CTRL and CTRL_EXT bits, values from Linux e1000e defines.h. Bits Linux
//! sets or clears by number in e1000_initialize_hw_bits_* are named for the
//! function that touches them.

pub const CTRL_GIO_MASTER_DISABLE: u32 = 1 << 2;
pub const CTRL_SLU: u32 = 1 << 6;
pub const CTRL_FRCSPD: u32 = 1 << 11;
pub const CTRL_FRCDPX: u32 = 1 << 12;
pub const CTRL_LANPHYPC_OVERRIDE: u32 = 1 << 16;
pub const CTRL_LANPHYPC_VALUE: u32 = 1 << 17;
pub const CTRL_RST: u32 = 1 << 26;
/// Cleared on 82573/82574/82583 by e1000_initialize_hw_bits_82571.
pub const CTRL_82574_HW_BIT29: u32 = 1 << 29;
pub const CTRL_PHY_RST: u32 = 1 << 31;

/// LCD Power Cycle Done: set once a LANPHYPC toggle has power-cycled the PHY.
pub const CTRL_EXT_LPCD: u32 = 1 << 2;
pub const CTRL_EXT_DPG_EN: u32 = 1 << 3;
pub const CTRL_EXT_FORCE_SMBUS: u32 = 1 << 11;
pub const CTRL_EXT_RO_DIS: u32 = 1 << 17;
pub const CTRL_EXT_PHYPDEN: u32 = 1 << 20;
/// Set by both e1000_initialize_hw_bits_ich8lan and _82571.
pub const CTRL_EXT_HW_BIT22: u32 = 1 << 22;
/// Cleared on 82573/82574/82583 by e1000_initialize_hw_bits_82571.
pub const CTRL_EXT_82574_HW_BIT23: u32 = 1 << 23;
/// Tells manageability firmware a driver owns the port (e1000e_get_hw_control).
pub const CTRL_EXT_DRV_LOAD: u32 = 1 << 28;
