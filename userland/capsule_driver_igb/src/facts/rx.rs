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

//! Receive control, split-receive control, queue control and write-back
//! bits. Source: as observed,
//! unverified against Intel's datasheets.

pub const RCTL_EN: u32 = 1 << 1;
pub const RCTL_SBP: u32 = 1 << 2;
pub const RCTL_LPE: u32 = 1 << 5;
/// Both loopback mode bits.
pub const RCTL_LBM_MASK: u32 = 0x3 << 6;
/// The multicast offset field.
pub const RCTL_MO_MASK: u32 = 0x3 << 12;
pub const RCTL_BAM: u32 = 1 << 15;
/// The buffer size field; SRRCTL sets the size per queue instead.
pub const RCTL_SZ_MASK: u32 = 0x3 << 16;
pub const RCTL_SECRC: u32 = 1 << 26;

/// SRRCTL: packet buffer size in 1 KiB units in 6:0, header buffer size in
/// 64-byte units from bit 8 (written as bytes << 2), descriptor type in
/// 27:25, one-buffer advanced.
pub const SRRCTL_DESCTYPE_ADV_ONEBUF: u32 = 1 << 25;
pub const SRRCTL_BSIZEPKT_2K: u32 = 2048 / 1024;
pub const SRRCTL_BSIZEHDR_256: u32 = 256 << 2;

/// RXDCTL thresholds. Prefetch and host thresholds of 8 are as observed,
/// unverified. The write-back threshold is this driver's: 1,
/// so each frame is written back alone. The driver polls with every cause
/// masked, and nothing would push a partial batch out.
pub const RXDCTL_PTHRESH: u32 = 8;
pub const RXDCTL_HTHRESH: u32 = 8 << 8;
pub const RXDCTL_WTHRESH: u32 = 1 << 16;
pub const RXDCTL_QUEUE_ENABLE: u32 = 1 << 25;

/// Write-back status: descriptor done, end of packet, and the frame error
/// bits (CRC, symbol, sequence, carrier extension, RX data error).
pub const RXD_STAT_DD: u32 = 1 << 0;
pub const RXD_STAT_EOP: u32 = 1 << 1;
pub const RXDEXT_ERR_FRAME_ERR_MASK: u32 =
    (1 << 24) | (1 << 25) | (1 << 26) | (1 << 28) | (1 << 31);
