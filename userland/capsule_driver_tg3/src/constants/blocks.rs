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

//! The send and receive engines' mode registers, in tg3_reset_hw's order of
//! enabling them (tg3.h).

pub const RDMAC_MODE: usize = 0x4800;
/// ENABLE | TGTABORT | MSTABORT | PARITYERR | ADDROFLOW | FIFOOFLOW |
/// FIFOURUN | FIFOOREAD | LNGREAD, tg3_reset_hw's starting rdmac_mode.
pub const RDMAC_MODE_BASE: u32 = 0x0000_03FE;
pub const RDMAC_MODE_FIFO_LONG_BURST: u32 = 0x0003_0000;
pub const RDMAC_MODE_JMB_2K_MMRR: u32 = 0x0080_0000;
pub const RDMAC_MODE_IPV4_LSO_EN: u32 = 0x0800_0000;
pub const RDMAC_MODE_IPV6_LSO_EN: u32 = 0x1000_0000;
pub const TG3_RDMA_RSRVCTRL_REG: usize = 0x4900;
pub const TG3_RDMA_RSRVCTRL_FIFO_OFLW_FIX: u32 = 0x0000_0004;
pub const WDMAC_MODE: usize = 0x4C00;
/// The same nine bits in the write engine (WDMAC_MODE_*).
pub const WDMAC_MODE_BASE: u32 = 0x0000_03FE;
pub const WDMAC_MODE_STATUS_TAG_FIX: u32 = 0x2000_0000;

pub const RCVLPC_MODE: usize = 0x2000;
pub const RCVLPC_CONFIG: usize = 0x2010;
pub const RCVLPC_STATSCTRL: usize = 0x2014;
pub const RCVLPC_STATSCTRL_ENABLE: u32 = 0x0000_0001;
pub const RCVLPC_STATS_ENABLE: usize = 0x2018;
pub const RCVLPC_STATSENAB_DACK_FIX: u32 = 0x0004_0000;
pub const RCVDBDI_MODE: usize = 0x2400;
pub const RCVDBDI_MODE_INV_RING_SZ: u32 = 0x0000_0010;
pub const RCVDCC_MODE: usize = 0x2800;
pub const RCVBDI_MODE: usize = 0x2C00;
pub const RCVCC_MODE: usize = 0x3000;
pub const SNDDATAI_MODE: usize = 0x0C00;
pub const SNDDATAI_STATSCTRL: usize = 0x0C08;
pub const SNDDATAI_STATSENAB: usize = 0x0C0C;
/// SNDDATAI_SCTRL_ENABLE | SNDDATAI_SCTRL_FASTUPD.
pub const SNDDATAI_SCTRL: u32 = 0x0000_0003;
/// The bit tg3_reset_hw adds to SNDDATAI_MODE, as a literal, for HW_TSO_3.
pub const SNDDATAI_MODE_HW_TSO: u32 = 0x0000_0008;
pub const SNDDATAC_MODE: usize = 0x1000;
pub const SNDBDS_MODE: usize = 0x1400;
pub const SNDBDI_MODE: usize = 0x1800;
pub const SNDBDC_MODE: usize = 0x1C00;
/// Every block's enable bit, and the attention bit most of them share.
pub const MODE_ENABLE: u32 = 0x0000_0002;
pub const MODE_ATTN_ENABLE: u32 = 0x0000_0004;
