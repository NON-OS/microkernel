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

//! Receive and transmit control bits, values from Linux e1000e defines.h
//! and ich8lan.h.

pub const RCTL_EN: u32 = 1 << 1;
pub const RCTL_BAM: u32 = 1 << 15;
/// BSIZE 00 with BSEX clear: 2048-byte buffers (E1000_RCTL_SZ_2048).
pub const RCTL_SZ_2048: u32 = 0;
pub const RCTL_SECRC: u32 = 1 << 26;
/// E1000_RCTL_RDMTS_HEX, which the SPT errata also writes into IOSFPC.
pub const RCTL_RDMTS_HEX: u32 = 1 << 16;

pub const RFCTL_NFSW_DIS: u32 = 1 << 6;
pub const RFCTL_NFSR_DIS: u32 = 1 << 7;
pub const RFCTL_EXTEN: u32 = 1 << 15;

pub const RXDCTL_THRESH_UNIT_DESC: u32 = 1 << 24;

pub const TCTL_EN: u32 = 1 << 1;
pub const TCTL_PSP: u32 = 1 << 3;
pub const TCTL_CT_MASK: u32 = 0xFF << 4;
pub const TCTL_COLD_MASK: u32 = 0x3FF << 12;
pub const TCTL_MULR: u32 = 1 << 28;
pub const TCTL_RTLC: u32 = 1 << 24;
/// E1000_COLLISION_THRESHOLD 15 and E1000_COLLISION_DISTANCE 63, as
/// e1000_configure_tx and e1000e_config_collision_dist_generic place them.
pub const TCTL_CT_DEFAULT: u32 = 15 << 4;
pub const TCTL_COLD_DEFAULT: u32 = 63 << 12;

pub const TXDCTL_PTHRESH: u32 = 0x0000_003F;
pub const TXDCTL_WTHRESH: u32 = 0x003F_0000;
pub const TXDCTL_FULL_TX_DESC_WB: u32 = 0x0101_0000;
pub const TXDCTL_MAX_TX_DESC_PREFETCH: u32 = 0x0100_001F;
/// E1000_TXDCTL_COUNT_DESC; both initialize_hw_bits functions set it as BIT(22).
pub const TXDCTL_COUNT_DESC: u32 = 1 << 22;

pub const TARC0_CB_MULTIQ_3_REQ: u32 = 0x3000_0000;
pub const TARC0_CB_MULTIQ_2_REQ: u32 = 0x2000_0000;

pub const GCR_HW_BIT22: u32 = 1 << 22;
pub const GCR_L1_ACT_WITHOUT_L0S_RX: u32 = 1 << 27;

pub const RAH_AV: u32 = 1 << 31;
