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
pub const IC_DATA_CMD_READ: u32 = 1 << 8;
pub const IC_DATA_CMD_STOP: u32 = 1 << 9;
pub const IC_DATA_CMD_RESTART: u32 = 1 << 10;
pub const IC_INTR_TX_ABRT: u32 = 1 << 6;
pub const IC_STATUS_TFE: u32 = 1 << 2;
pub const IC_STATUS_RFNE: u32 = 1 << 3;
pub const IC_STATUS_MST_ACTIVITY: u32 = 1 << 5;
pub const IC_ENABLE_ENABLE: u32 = 1;
// IC_ENABLE.ABORT: the master issues STOP and flushes its FIFO; the core
// clears the bit when done. Needed to disable a master holding the bus.
pub const IC_ENABLE_ABORT: u32 = 1 << 1;
pub const IC_INTR_MST_ON_HOLD: u32 = 1 << 13;
// IC_TX_ABRT_SOURCE: the address or a data byte went unacknowledged (the
// device is absent or refused), and arbitration was lost to another master.
pub const ABRT_NOACK_MASK: u32 = 0x1F;
pub const ABRT_GCALL_NOACK: u32 = 1 << 5;
pub const ABRT_LOST: u32 = 1 << 12;
// IC_COMP_TYPE of every DesignWare I2C core ("DW" 0x0140), as Linux
// i2c_dw_check_component_type requires it.
pub const IC_COMP_TYPE_VALUE: u32 = 0x4457_0140;
// IC_SDA_HOLD bits 23:16 hold SDA after SCL falls while receiving; Linux sets
// one cycle when firmware leaves it zero.
pub const IC_SDA_HOLD_RX_SHIFT: u32 = 16;
pub const IC_CLR_TX_ABRT: u64 = 0x54;
// IC_COMP_PARAM_1 carries each FIFO's depth minus one in an eight-bit field.
pub const COMP_PARAM_TX_DEPTH_SHIFT: u32 = 16;
pub const COMP_PARAM_RX_DEPTH_SHIFT: u32 = 8;
pub const IC_CON_MASTER_MODE: u32 = 1 << 0;
pub const IC_CON_SPEED_STD: u32 = 1 << 1;
pub const IC_CON_SPEED_FAST: u32 = 2 << 1;
pub const IC_CON_RESTART_EN: u32 = 1 << 5;
pub const IC_CON_SLAVE_DISABLE: u32 = 1 << 6;
// Time bounds, in milliseconds of uptime, not in loop iterations: an MMIO
// read costs a few microseconds on QEMU and a few hundred nanoseconds on an
// LPSS function, so an iteration count that is ample on one is short on the
// other. Linux allows the disable 100 retries of 25 to 250 us.
pub const ENABLE_TIMEOUT_MS: u64 = 25;
pub const IDLE_TIMEOUT_MS: u64 = 50;
// A transfer gets a base plus a per-byte allowance at the slowest bus speed
// (standard mode moves a byte in about 90 us) with room for clock stretching.
pub const TRANSFER_BASE_MS: u64 = 50;
pub const TRANSFER_PER_KIB_MS: u64 = 250;
