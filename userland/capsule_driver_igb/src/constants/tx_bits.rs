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
//! Transmit control, queue control and advanced data descriptor bits:
//! igb e1000_defines.h, igb.h and e1000_82575.h.

pub const TCTL_EN: u32 = 1 << 1;
pub const TCTL_PSP: u32 = 1 << 3;
pub const TCTL_CT_MASK: u32 = 0xFF << 4;
/// E1000_COLLISION_THRESHOLD (15) << E1000_CT_SHIFT (4), as igb_setup_tctl.
pub const TCTL_CT_15: u32 = 15 << 4;
pub const TCTL_RTLC: u32 = 1 << 24;

/// igb.h IGB_TX_PTHRESH (8) and IGB_TX_HTHRESH (1). WTHRESH is 1, not igb's
/// 16: above one the part may hold DD write-backs for a batch, and this
/// driver polls with every interrupt cause masked, so nothing is relied on
/// to push a partial batch out before the ring fills.
pub const TXDCTL_PTHRESH: u32 = 8;
pub const TXDCTL_HTHRESH: u32 = 1 << 8;
pub const TXDCTL_WTHRESH: u32 = 1 << 16;
pub const TXDCTL_QUEUE_ENABLE: u32 = 1 << 25;

/// e1000_82575.h E1000_ADVTXD_*: data descriptor type, the command bits
/// igb_tx_cmd_type and IGB_TXD_DCMD set, and the PAYLEN position in
/// olinfo_status.
pub const ADVTXD_DTYP_DATA: u32 = 0x3 << 20;
pub const ADVTXD_DCMD_EOP: u32 = 1 << 24;
pub const ADVTXD_DCMD_IFCS: u32 = 1 << 25;
pub const ADVTXD_DCMD_RS: u32 = 1 << 27;
pub const ADVTXD_DCMD_DEXT: u32 = 1 << 29;
pub const ADVTXD_PAYLEN_SHIFT: u32 = 14;
/// The buffer length sits in the low 16 bits of cmd_type_len.
pub const ADVTXD_LEN_MASK: u32 = 0xFFFF;
/// E1000_TXD_STAT_DD in the write-back status.
pub const TXD_STAT_DD: u32 = 1 << 0;
