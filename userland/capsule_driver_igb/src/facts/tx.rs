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

//! Transmit control, queue control and advanced descriptor bits. Source:
//! observed in Linux igb (e1000_defines.h, e1000_82575.h), unverified
//! against Intel's datasheets.

pub const TCTL_EN: u32 = 1 << 1;
/// Pad short packets to the minimum length.
pub const TCTL_PSP: u32 = 1 << 3;
pub const TCTL_CT_MASK: u32 = 0xFF << 4;
/// Collision threshold 15, the half-duplex retry count IEEE 802.3 sets.
pub const TCTL_CT_15: u32 = 15 << 4;
/// Retransmit on late collision.
pub const TCTL_RTLC: u32 = 1 << 24;

/// TXDCTL thresholds: prefetch 8 and host 1 as observed in Linux igb.h,
/// unverified; write-back 1 is this driver's, for the reason in rx.rs.
pub const TXDCTL_PTHRESH: u32 = 8;
pub const TXDCTL_HTHRESH: u32 = 1 << 8;
pub const TXDCTL_WTHRESH: u32 = 1 << 16;
pub const TXDCTL_QUEUE_ENABLE: u32 = 1 << 25;

/// Advanced data descriptor: type, end of packet, insert FCS, report
/// status, extended; the payload length's place in olinfo_status.
pub const ADVTXD_DTYP_DATA: u32 = 0x3 << 20;
pub const ADVTXD_DCMD_EOP: u32 = 1 << 24;
pub const ADVTXD_DCMD_IFCS: u32 = 1 << 25;
pub const ADVTXD_DCMD_RS: u32 = 1 << 27;
pub const ADVTXD_DCMD_DEXT: u32 = 1 << 29;
pub const ADVTXD_PAYLEN_SHIFT: u32 = 14;
/// The buffer length, in the low 16 bits of cmd_type_len.
pub const ADVTXD_LEN_MASK: u32 = 0xFFFF;
/// Descriptor done, in the write-back status.
pub const TXD_STAT_DD: u32 = 1 << 0;
