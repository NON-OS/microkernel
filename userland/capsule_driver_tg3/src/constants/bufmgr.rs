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

//! The buffer manager and the receive BD thresholds (tg3.h, tg3.c), with
//! tg3_init_bufmgr_config's 57765_PLUS watermarks.

pub const BUFMGR_MODE: usize = 0x4400;
/// BUFMGR_MODE_ENABLE | BUFMGR_MODE_ATTN_ENABLE.
pub const BUFMGR_MODE_ON: u32 = 0x0000_0006;
pub const BUFMGR_MODE_ENABLE: u32 = 0x0000_0002;
pub const BUFMGR_MB_RDMA_LOW_WATER: usize = 0x4410;
pub const BUFMGR_MB_MACRX_LOW_WATER: usize = 0x4414;
pub const BUFMGR_MB_HIGH_WATER: usize = 0x4418;
pub const BUFMGR_DMA_LOW_WATER: usize = 0x4434;
pub const BUFMGR_DMA_HIGH_WATER: usize = 0x4438;
pub const DEFAULT_MB_RDMA_LOW_WATER_5705: u32 = 0x00;
pub const DEFAULT_MB_MACRX_LOW_WATER_57765: u32 = 0x2A;
pub const DEFAULT_MB_HIGH_WATER_57765: u32 = 0xA0;
pub const DEFAULT_DMA_LOW_WATER: u32 = 0x05;
pub const DEFAULT_DMA_HIGH_WATER: u32 = 0x0A;

pub const RCVBDI_STD_THRESH: usize = 0x2C18;
pub const RCVBDI_JUMBO_THRESH: usize = 0x2C1C;
pub const STD_REPLENISH_LWM: usize = 0x2D00;
pub const JMB_REPLENISH_LWM: usize = 0x2D04;
pub const TG3_SRAM_RX_STD_BDCACHE_SIZE_5700: u32 = 128;
pub const TG3_SRAM_RX_JMB_BDCACHE_SIZE_5700: u32 = 64;
/// TG3_DEF_RX_JUMBO_RING_PENDING, which the jumbo threshold is worked from
/// even with the jumbo ring disabled.
pub const TG3_DEF_RX_JUMBO_RING_PENDING: u32 = 100;
