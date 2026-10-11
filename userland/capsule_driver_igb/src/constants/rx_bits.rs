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
//! Receive control, split-and-replication control, queue control and
//! write-back bits: igb e1000_defines.h, e1000_82575.h and igb.h.

pub const RCTL_EN: u32 = 1 << 1;
pub const RCTL_SBP: u32 = 1 << 2;
pub const RCTL_LPE: u32 = 1 << 5;
/// E1000_RCTL_LBM_TCVR | E1000_RCTL_LBM_MAC: both loopback bits.
pub const RCTL_LBM_MASK: u32 = 0x3 << 6;
/// 3 << E1000_RCTL_MO_SHIFT, the multicast offset igb_setup_rctl clears.
pub const RCTL_MO_MASK: u32 = 0x3 << 12;
pub const RCTL_BAM: u32 = 1 << 15;
/// E1000_RCTL_SZ_256 covers both size bits; SRRCTL sets the size per queue.
pub const RCTL_SZ_MASK: u32 = 0x3 << 16;
pub const RCTL_SECRC: u32 = 1 << 26;

/// e1000_82575.h: BSIZEPKT in 1 KiB units in bits 6:0, the header size in
/// 64-byte units in bits 11:8, the descriptor type in bits 27:25.
pub const SRRCTL_DESCTYPE_ADV_ONEBUF: u32 = 1 << 25;
pub const SRRCTL_BSIZEPKT_2K: u32 = 2048 / 1024;
/// IGB_RX_HDR_LEN (256) << E1000_SRRCTL_BSIZEHDRSIZE_SHIFT (2), as
/// igb_configure_rx_ring writes it.
pub const SRRCTL_BSIZEHDR_256: u32 = 256 << 2;

/// igb.h IGB_RX_PTHRESH and IGB_RX_HTHRESH. WTHRESH is 1, not igb's 4: above
/// one the part may hold finished descriptors back to write them as a batch,
/// and this driver polls with every interrupt cause masked, so nothing is
/// relied on to push a partial batch out. One writes each frame back alone.
pub const RXDCTL_PTHRESH: u32 = 8;
pub const RXDCTL_HTHRESH: u32 = 8 << 8;
pub const RXDCTL_WTHRESH: u32 = 1 << 16;
pub const RXDCTL_QUEUE_ENABLE: u32 = 1 << 25;

/// Write-back status_error: E1000_RXD_STAT_DD and _EOP, and
/// E1000_RXDEXT_ERR_FRAME_ERR_MASK (CE, SE, SEQ, CXE and RXE), what
/// igb_cleanup_headers drops a frame on.
pub const RXD_STAT_DD: u32 = 1 << 0;
pub const RXD_STAT_EOP: u32 = 1 << 1;
pub const RXDEXT_ERR_FRAME_ERR_MASK: u32 =
    (1 << 24) | (1 << 25) | (1 << 26) | (1 << 28) | (1 << 31);
