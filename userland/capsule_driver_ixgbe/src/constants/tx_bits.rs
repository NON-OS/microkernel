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

//! Transmit queue and descriptor bits, from Linux ixgbe_type.h and
//! ixgbe_main.c.

/// ixgbe_configure_tx_ring: PTHRESH 32 ("avoids a TX hang with DFP
/// enabled"), HTHRESH 1, and WTHRESH 1, its value without interrupt
/// moderation, which a polled driver has none of.
pub const TXDCTL_PTHRESH: u32 = 32;
pub const TXDCTL_HTHRESH: u32 = 1 << 8;
pub const TXDCTL_WTHRESH: u32 = 1 << 16;
pub const TXDCTL_ENABLE: u32 = 1 << 25;

/// IXGBE_ADVTXD_*: data descriptor type, the command bits ixgbe_tx_cmd_type
/// and IXGBE_TXD_CMD set, and the PAYLEN position in olinfo_status.
pub const ADVTXD_DTYP_DATA: u32 = 0x3 << 20;
pub const ADVTXD_DCMD_EOP: u32 = 1 << 24;
pub const ADVTXD_DCMD_IFCS: u32 = 1 << 25;
pub const ADVTXD_DCMD_RS: u32 = 1 << 27;
pub const ADVTXD_DCMD_DEXT: u32 = 1 << 29;
pub const ADVTXD_PAYLEN_SHIFT: u32 = 14;
/// The buffer length sits in the low 16 bits of cmd_type_len.
pub const ADVTXD_LEN_MASK: u32 = 0xFFFF;
/// IXGBE_TXD_STAT_DD in the write-back status.
pub const TXD_STAT_DD: u32 = 1 << 0;
