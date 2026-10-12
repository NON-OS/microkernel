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

//! Receive queue and write-back bits. Source: observed in Linux ixgbe
//! headers, unverified against Intel's datasheets.

/// SRRCTL: the packet buffer in 1 KiB units in bits 4:0, the header buffer
/// in 64-byte units from bit 8 (written as bytes << 2), the descriptor type
/// in 27:25, one-buffer advanced.
pub const SRRCTL_DESCTYPE_ADV_ONEBUF: u32 = 1 << 25;
pub const SRRCTL_BSIZEPKT_2K: u32 = 2048 >> 10;
/// A 256-byte header buffer size, unused with one buffer per descriptor.
pub const SRRCTL_BSIZEHDR_256: u32 = 256 << 2;

/// Only the enable is written; the thresholds stay as the reset leaves them.
pub const RXDCTL_ENABLE: u32 = 1 << 25;

/// Write-back status: descriptor done, end of packet, and the frame error
/// bits (CRC, length, byte, oversize and undersize), any of which drops the
/// frame.
pub const RXD_STAT_DD: u32 = 1 << 0;
pub const RXD_STAT_EOP: u32 = 1 << 1;
pub const RXDEXT_ERR_FRAME_ERR_MASK: u32 =
    (1 << 24) | (1 << 25) | (1 << 27) | (1 << 28) | (1 << 29);
