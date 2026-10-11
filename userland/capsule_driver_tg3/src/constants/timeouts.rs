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

//! How long each wait lasts: the loop count of the Linux wait times its
//! longest delay, so this driver never gives up sooner than tg3 would. A
//! microsecond delay Linux takes between two writes is taken here as a
//! millisecond hold, the shortest the scheduler offers.

/// tg3_nvram_lock: 8000 passes of udelay(20).
pub const NVRAM_LOCK_MS: u64 = 160;
/// tg3_poll_fw: 100000 passes of udelay(10).
pub const POLL_FW_MS: u64 = 1000;
/// tg3_chip_reset's two udelay(120) around the posted-write flush.
pub const CORE_RESET_HOLD_MS: u64 = 1;
/// tg3_reset_hw's BUFMGR enable and HOSTCC disable: 2000 of udelay(10).
pub const BLOCK_MODE_MS: u64 = 20;
/// tg3_stop_block and the TX_MODE wait: MAX_WAIT_CNT (1000) of udelay(100).
pub const STOP_BLOCK_MS: u64 = 100;
/// PHY_BUSY_LOOPS (5000) passes of udelay(10), for one MDIO frame.
pub const MDIO_MS: u64 = 50;
/// tg3_bmcr_reset: 5000 passes of udelay(10).
pub const BMCR_RESET_MS: u64 = 50;
/// TG3_FW_EVENT_TIMEOUT_USEC, rounded up.
pub const FW_EVENT_MS: u64 = 3;
/// tg3_setup_copper_phy: link status, 100 of udelay(40); aux status, 2000
/// of udelay(10); BMCR read back, 200 of udelay(10).
pub const LINK_UP_MS: u64 = 4;
pub const AUX_STAT_MS: u64 = 20;
pub const BMCR_SETTLE_MS: u64 = 2;
/// TG3_FW_UPDATE_FREQ_SEC: the ASF heartbeat period.
pub const ASF_HEARTBEAT_MS: u64 = 2000;
/// tg3_timer's period while the link is polled (TG3_TIMER_RATE is HZ).
pub const TICK_MS: u64 = 1000;
