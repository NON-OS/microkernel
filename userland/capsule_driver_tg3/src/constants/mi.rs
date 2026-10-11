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

//! MDIO through MAC_MI_COM, and the LED modes (tg3.h).

pub const MAC_MI_COM: usize = 0x044C;
pub const MI_COM_CMD_WRITE: u32 = 0x0400_0000;
pub const MI_COM_CMD_READ: u32 = 0x0800_0000;
pub const MI_COM_START: u32 = 0x2000_0000;
pub const MI_COM_BUSY: u32 = 0x2000_0000;
pub const MI_COM_PHY_ADDR_MASK: u32 = 0x03E0_0000;
pub const MI_COM_PHY_ADDR_SHIFT: u32 = 21;
pub const MI_COM_REG_ADDR_MASK: u32 = 0x001F_0000;
pub const MI_COM_REG_ADDR_SHIFT: u32 = 16;
pub const MI_COM_DATA_MASK: u32 = 0x0000_FFFF;
pub const MAC_MI_STAT: usize = 0x0450;
pub const MAC_MI_STAT_LNKSTAT_ATTN_ENAB: u32 = 0x0000_0001;
pub const MAC_MI_MODE: usize = 0x0454;
pub const MAC_MI_MODE_AUTO_POLL: u32 = 0x0000_0010;
/// tp->mi_mode for a part with a CPMU (tg3_get_invariants).
pub const MAC_MI_MODE_500KHZ_CONST: u32 = 0x0000_8000;
/// TG3_PHY_MII_ADDR: the internal PHY of every part before 5717_PLUS.
pub const TG3_PHY_MII_ADDR: u32 = 0x01;

pub const LED_CTRL_MODE_MAC: u32 = 0x0000_0000;
pub const LED_CTRL_MODE_PHY_1: u32 = 0x0000_0800;
pub const LED_CTRL_MODE_PHY_2: u32 = 0x0000_1000;
pub const LED_CTRL_MODE_SHASTA_MAC: u32 = 0x0000_2000;
pub const LED_CTRL_MODE_SHARED: u32 = 0x0000_4000;
pub const LED_CTRL_MODE_COMBO: u32 = 0x0000_8000;
