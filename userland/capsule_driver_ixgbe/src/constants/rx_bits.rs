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

//! Receive queue and write-back bits, from Linux ixgbe_type.h and ixgbe.h.

/// SRRCTL: the packet buffer in 1 KiB units in bits 4:0
/// (IXGBE_SRRCTL_BSIZEPKT_SHIFT 10), the header buffer in 64-byte units from
/// bit 8 (IXGBE_SRRCTL_BSIZEHDRSIZE_SHIFT 2), the descriptor type in 27:25.
pub const SRRCTL_DESCTYPE_ADV_ONEBUF: u32 = 1 << 25;
pub const SRRCTL_BSIZEPKT_2K: u32 = 2048 >> 10;
/// IXGBE_RX_HDR_SIZE (256) << IXGBE_SRRCTL_BSIZEHDRSIZE_SHIFT, as
/// ixgbe_configure_srrctl writes it.
pub const SRRCTL_BSIZEHDR_256: u32 = 256 << 2;

/// ixgbe_configure_rx_ring sets only the enable on the 82599 and later;
/// the thresholds stay as the reset leaves them.
pub const RXDCTL_ENABLE: u32 = 1 << 25;

/// Write-back status_error: IXGBE_RXD_STAT_DD and _EOP, and
/// IXGBE_RXDADV_ERR_FRAME_ERR_MASK (CE, LE, PE, OSE and USE), what
/// ixgbe_cleanup_headers drops a frame on.
pub const RXD_STAT_DD: u32 = 1 << 0;
pub const RXD_STAT_EOP: u32 = 1 << 1;
pub const RXDEXT_ERR_FRAME_ERR_MASK: u32 =
    (1 << 24) | (1 << 25) | (1 << 27) | (1 << 28) | (1 << 29);
