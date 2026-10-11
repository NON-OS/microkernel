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

//! The MAC block (tg3.h, "MAC control registers").

pub const MAC_MODE: usize = 0x0400;
pub const MAC_MODE_HALF_DUPLEX: u32 = 0x0000_0002;
pub const MAC_MODE_PORT_MODE_MASK: u32 = 0x0000_000C;
pub const MAC_MODE_PORT_MODE_MII: u32 = 0x0000_0004;
pub const MAC_MODE_PORT_MODE_GMII: u32 = 0x0000_0008;
pub const MAC_MODE_RXSTAT_ENABLE: u32 = 0x0000_0800;
pub const MAC_MODE_RXSTAT_CLEAR: u32 = 0x0000_1000;
pub const MAC_MODE_TXSTAT_ENABLE: u32 = 0x0000_4000;
pub const MAC_MODE_TXSTAT_CLEAR: u32 = 0x0000_8000;
pub const MAC_MODE_TDE_ENABLE: u32 = 0x0020_0000;
pub const MAC_MODE_RDE_ENABLE: u32 = 0x0040_0000;
pub const MAC_MODE_FHDE_ENABLE: u32 = 0x0080_0000;
pub const MAC_STATUS: usize = 0x0404;
pub const MAC_STATUS_CFG_CHANGED: u32 = 0x0000_0008;
pub const MAC_STATUS_SYNC_CHANGED: u32 = 0x0000_0010;
pub const MAC_STATUS_LNKSTATE_CHANGED: u32 = 0x0000_1000;
pub const MAC_STATUS_MI_COMPLETION: u32 = 0x0040_0000;
pub const MAC_EVENT: usize = 0x0408;
pub const MAC_EVENT_LNKSTATE_CHANGED: u32 = 0x0000_1000;
pub const MAC_LED_CTRL: usize = 0x040C;
pub const MAC_ADDR_0_HIGH: usize = 0x0410;
pub const MAC_ADDR_0_LOW: usize = 0x0414;
pub const MAC_TX_BACKOFF_SEED: usize = 0x0438;
pub const TX_BACKOFF_SEED_MASK: u32 = 0x0000_03FF;
pub const MAC_RX_MTU_SIZE: usize = 0x043C;
pub const MAC_TX_MODE: usize = 0x045C;
pub const TX_MODE_ENABLE: u32 = 0x0000_0002;
pub const TX_MODE_FLOW_CTRL_ENABLE: u32 = 0x0000_0010;
pub const TX_MODE_MBUF_LOCKUP_FIX: u32 = 0x0000_0100;
pub const MAC_TX_LENGTHS: usize = 0x0464;
pub const TX_LENGTHS_SLOT_TIME_SHIFT: u32 = 0;
pub const TX_LENGTHS_IPG_SHIFT: u32 = 8;
pub const TX_LENGTHS_IPG_CRS_SHIFT: u32 = 12;
pub const MAC_RX_MODE: usize = 0x0468;
pub const RX_MODE_ENABLE: u32 = 0x0000_0002;
pub const RX_MODE_FLOW_CTRL_ENABLE: u32 = 0x0000_0004;
pub const RX_MODE_PROMISC: u32 = 0x0000_0100;
pub const RX_MODE_KEEP_VLAN_TAG: u32 = 0x0000_0400;
pub const RX_MODE_IPV6_CSUM_ENABLE: u32 = 0x0100_0000;
pub const MAC_HASH_REG_0: usize = 0x0470;
pub const MAC_RCV_RULE_0: usize = 0x0480;
pub const MAC_RCV_VALUE_0: usize = 0x0484;
pub const RCV_RULE_DISABLE_MASK: u32 = 0x7FFF_FFFF;
pub const MAC_RCV_RULE_CFG: usize = 0x0500;
pub const RCV_RULE_CFG_DEFAULT_CLASS: u32 = 0x0000_0008;
pub const MAC_LOW_WMARK_MAX_RX_FRAME: usize = 0x0504;
