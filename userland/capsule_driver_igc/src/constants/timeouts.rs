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

//! Every wait, in milliseconds of uptime. Linux counts loop passes with a
//! sleep in each; the bound here is that count times the longest sleep in
//! the range, so this driver never gives up on a part sooner than Linux
//! might. The error strings that quote a bound are checked against these
//! values in igc_proofs.

/// igc_disable_pcie_master: MASTER_DISABLE_TIMEOUT (800) passes of
/// usleep_range(2000, 3000).
pub const MASTER_DISABLE_MS: u64 = 2400;
/// igc_reset_hw_base: usleep_range(10000, 20000) after RCTL and TCTL are
/// written and before CTRL.RST.
pub const QUIESCE_SETTLE_MS: u64 = 10;
/// igc_get_auto_rd_done: AUTO_READ_DONE_TIMEOUT (10) passes of
/// usleep_range(1000, 2000).
pub const AUTO_READ_MS: u64 = 20;
/// igc_get_hw_semaphore_i225: nvm.word_size + 1 passes of
/// usleep_range(500, 600) per bit. The word size depends on the NVM
/// fitted; one second is taken here for each of SMBI and SWESMBI.
pub const SEMAPHORE_MS: u64 = 1000;
/// igc_acquire_swfw_sync_i225: 200 passes with mdelay(5) between.
pub const SWFW_SYNC_MS: u64 = 1000;
pub const SWFW_RETRY_MS: u64 = 5;
/// igc_read_phy_reg_mdic: IGC_GEN_POLL_TIMEOUT (1920) passes of udelay(50).
pub const MDIC_MS: u64 = 100;
/// Queue enable read-back. Linux writes RXDCTL/TXDCTL and does not read them
/// back; igc_rx_fifo_flush_base gives a queue disable 10 ms. Ten times that
/// is allowed here for the enable to show.
pub const QUEUE_ENABLE_MS: u64 = 100;
