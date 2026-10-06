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
//! write-back bits: igc_defines.h, igc_base.h and igc.h.

pub const RCTL_EN: u32 = 1 << 1;
pub const RCTL_SBP: u32 = 1 << 2;
pub const RCTL_LPE: u32 = 1 << 5;
/// IGC_RCTL_LBM_TCVR covers both loopback bits.
pub const RCTL_LBM_MASK: u32 = 0x3 << 6;
/// 3 << IGC_RCTL_MO_SHIFT, the multicast offset igc_setup_rctl clears.
pub const RCTL_MO_MASK: u32 = 0x3 << 12;
pub const RCTL_BAM: u32 = 1 << 15;
/// IGC_RCTL_SZ_256 covers both size bits; clear means 2048, and SRRCTL
/// overrides it per queue anyway.
pub const RCTL_SZ_MASK: u32 = 0x3 << 16;
pub const RCTL_SECRC: u32 = 1 << 26;

/// igc_base.h: BSIZEPKT in 1 KiB units, BSIZEHDR in 64-byte units.
pub const SRRCTL_BSIZEPKT_MASK: u32 = 0x7F;
pub const SRRCTL_BSIZEHDR_MASK: u32 = 0x3F << 8;
pub const SRRCTL_DESCTYPE_MASK: u32 = 0x7 << 25;
pub const SRRCTL_DESCTYPE_ADV_ONEBUF: u32 = 1 << 25;
pub const SRRCTL_BSIZEPKT_2K: u32 = 2048 / 1024;
/// IGC_SRRCTL_BSIZEHDR(IGC_RX_HDR_LEN), 256 bytes, as igc_configure_rx_ring.
pub const SRRCTL_BSIZEHDR_256: u32 = (256 / 64) << 8;

/// igc.h IGC_RXDCTL_PTHRESH and HTHRESH. WTHRESH is 1, not igc's 4: above
/// one the part may hold finished descriptors back to write them as a batch,
/// and this driver polls with every interrupt cause masked, so nothing is
/// relied on to push a partial batch out. One writes each frame back alone.
pub const RXDCTL_PTHRESH: u32 = 8;
pub const RXDCTL_HTHRESH: u32 = 8 << 8;
pub const RXDCTL_WTHRESH: u32 = 1 << 16;
pub const RXDCTL_QUEUE_ENABLE: u32 = 1 << 25;

/// Write-back status_error: IGC_RXD_STAT_DD, IGC_RXD_STAT_EOP and
/// IGC_RXDEXT_STATERR_RXE.
pub const RXD_STAT_DD: u32 = 1 << 0;
pub const RXD_STAT_EOP: u32 = 1 << 1;
pub const RXDEXT_STATERR_RXE: u32 = 1 << 31;
