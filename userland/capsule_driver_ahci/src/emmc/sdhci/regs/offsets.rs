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

//! Register offsets from the slot base, and the size of one slot window.

pub const SLOT_WINDOW: u64 = 0x100;

pub const BLOCK_SIZE: u32 = 0x04;
pub const BLOCK_COUNT: u32 = 0x06;
pub const ARGUMENT: u32 = 0x08;
pub const TRANSFER_MODE: u32 = 0x0c;
pub const COMMAND: u32 = 0x0e;
pub const RESPONSE: u32 = 0x10;
pub const PRESENT_STATE: u32 = 0x24;
pub const HOST_CONTROL: u32 = 0x28;
pub const POWER_CONTROL: u32 = 0x29;
pub const CLOCK_CONTROL: u32 = 0x2c;
pub const TIMEOUT_CONTROL: u32 = 0x2e;
pub const SOFTWARE_RESET: u32 = 0x2f;
/// Normal Interrupt Status in bits 15:0, Error Interrupt Status in 31:16.
pub const INT_STATUS: u32 = 0x30;
/// Normal Interrupt Status Enable in 15:0, Error in 31:16.
pub const INT_ENABLE: u32 = 0x34;
/// Normal Interrupt Signal Enable in 15:0, Error in 31:16.
pub const SIGNAL_ENABLE: u32 = 0x38;
pub const AUTO_CMD_ERROR: u32 = 0x3c;
pub const HOST_CONTROL2: u32 = 0x3e;
pub const CAPABILITIES: u32 = 0x40;
pub const CAPABILITIES_1: u32 = 0x44;
pub const ADMA_ERROR: u32 = 0x54;
pub const ADMA_ADDRESS: u32 = 0x58;
pub const ADMA_ADDRESS_HI: u32 = 0x5c;
pub const HOST_VERSION: u32 = 0xfe;
