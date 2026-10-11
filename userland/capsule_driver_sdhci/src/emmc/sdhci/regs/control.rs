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

//! Present State, Host Control 1, Power, Clock, Timeout and Software
//! Reset register fields.

// Present State.
pub const PS_CMD_INHIBIT: u32 = 1 << 0;
pub const PS_DAT_INHIBIT: u32 = 1 << 1;
pub const PS_CARD_INSERTED: u32 = 1 << 16;
pub const PS_CARD_STABLE: u32 = 1 << 17;

// Host Control 1.
pub const HC_4BIT: u8 = 1 << 1;
pub const HC_HIGH_SPEED: u8 = 1 << 2;
pub const HC_DMA_MASK: u8 = 3 << 3;
pub const HC_ADMA2_32: u8 = 2 << 3;
pub const HC_ADMA2_64: u8 = 3 << 3;
pub const HC_8BIT: u8 = 1 << 5;
pub const HC_CD_TEST_LEVEL: u8 = 1 << 6;
pub const HC_CD_TEST_SELECT: u8 = 1 << 7;

// Power Control: bus voltage select in 3:1, SD Bus Power in 0.
pub const POWER_ON: u8 = 1 << 0;
pub const POWER_180: u8 = 5 << 1;
pub const POWER_300: u8 = 6 << 1;
pub const POWER_330: u8 = 7 << 1;

// Clock Control.
pub const CLK_INT_EN: u16 = 1 << 0;
pub const CLK_INT_STABLE: u16 = 1 << 1;
pub const CLK_CARD_EN: u16 = 1 << 2;

/// Timeout Control: TMCLK x 2^27, the longest data timeout the counter
/// offers (0xF is reserved).
pub const TIMEOUT_MAX: u8 = 0x0e;

// Software Reset.
pub const RESET_ALL: u8 = 1 << 0;
pub const RESET_CMD: u8 = 1 << 1;
pub const RESET_DATA: u8 = 1 << 2;
