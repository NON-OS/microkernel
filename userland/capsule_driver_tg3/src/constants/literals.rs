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

//! The numbers tg3_reset_hw and its helpers write as literals or work out
//! in place (tg3.c), named here so each has one home.

/// GRC_MISC_CFG's timer prescaler for the 66 MHz clock: `65 << 1`.
pub const PRESCALER_66MHZ: u32 = 65;
/// tg3_reset_hw: RCVLPC_CONFIG.
pub const RCVLPC_CONFIG_VALUE: u32 = 0x0181;
/// MAC_RX_MTU_SIZE: MTU + ETH_HLEN + ETH_FCS_LEN + VLAN_HLEN.
pub const RX_MTU_SIZE: u32 = 1500 + 14 + 4 + 4;
/// MAC_TX_LENGTHS: IPG CRS 2, IPG 6, slot time 32.
pub const TX_LENGTHS_IPG_CRS: u32 = 2;
pub const TX_LENGTHS_IPG: u32 = 6;
pub const TX_LENGTHS_SLOT_TIME: u32 = 32;
/// MAC_LOW_WMARK_MAX_RX_FRAME for the 57765 class (2 on other parts).
pub const LOW_WMARK_57765: u32 = 1;
/// SNDDATAI_STATSENAB: every send statistic.
pub const SNDDATAI_STATS_ALL: u32 = 0x00FF_FFFF;
/// The first two receive rules and their values, masked with
/// RCV_RULE_DISABLE_MASK as tg3_reset_hw writes them.
pub const RCV_RULE_0: u32 = 0xC200_0000;
pub const RCV_RULE_1: u32 = 0x8600_0004;
pub const RCV_VALUE_ALL: u32 = 0xFFFF_FFFF;
/// MAC_RCV_RULE_n and MAC_RCV_VALUE_n are 8 bytes after rule n - 1.
pub const RCV_RULE_STRIDE: usize = 8;
/// The rules a 5705_PLUS part has, and how many ASF firmware keeps.
pub const RCV_RULES: usize = 8;
pub const RCV_RULES_ASF: usize = 4;
/// Rules 2 and 3 are left as they are: tg3_reset_hw's writes to them are
/// commented out.
pub const RCV_RULES_KEPT: usize = 4;
