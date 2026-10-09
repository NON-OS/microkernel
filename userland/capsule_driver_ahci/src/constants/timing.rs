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

//! Every wait the driver makes, in milliseconds of the monotonic clock. A
//! spin count measured the speed of the CPU it ran on, and a slow Celeron
//! and a fast desktop disagreed by an order of magnitude; these do not.

/// GHC.HR self-clears within 1 s (AHCI 1.3.1, 10.4.3); still set after that,
/// the HBA is not usable.
pub const HBA_RESET_MS: u64 = 1_000;
/// BIOS/OS handoff: the BIOS clears BOHC.BOS within 25 ms of OOS being set
/// (AHCI 1.3.1, 10.6.3)...
pub const HANDOFF_BOS_MS: u64 = 25;
/// ...unless it is busy (BOHC.BB), when it has 2 s to finish.
pub const HANDOFF_BUSY_MS: u64 = 2_000;
/// PxCMD.CR and PxCMD.FR clear within 500 ms of ST and FRE (AHCI 1.3.1,
/// 10.1.2), and PxCMD.CLO within the same.
pub const ENGINE_STOP_MS: u64 = 500;
/// After the staggered spin-up, the most the walk waits for a link that is
/// mid-negotiation (DET = 1) to settle.
pub const SPIN_UP_SETTLE_MS: u64 = 1_000;
/// Hold the PHY in COMRESET this long before releasing it (at least 1 ms).
pub const COMRESET_HOLD_MS: u64 = 2;
/// After COMRESET, the most the PHY may take to report communication.
pub const LINK_TIMEOUT_MS: u64 = 2_000;
/// A port whose DET reads no device this long after COMRESET is empty.
pub const LINK_EMPTY_MS: u64 = 200;
/// The most a device may stay BSY after its link comes up: a spinning disk
/// spins up before it clears BSY, and some take several seconds.
pub const DEVICE_READY_MS: u64 = 10_000;
/// The same wait after a COMRESET that recovers a stuck port while a request
/// is being served, kept short so the reply stays within the kernel's wait.
pub const RECOVER_READY_MS: u64 = 1_000;
/// The most one command may take, issue to completion: 30 s, Linux's
/// ATA_TMOUT_INTERNAL and SCSI default. A drive busy with its own
/// housekeeping, or one waking from a low-power state, takes several seconds
/// on a write or a cache flush, and 3 s failed it there. The kernel waits
/// 35 s for a reply (services/lifecycle/reply_wait.rs SLOW_BUDGET_MS) and
/// the installer's client 65 s, so a command that times out still answers
/// with the port recovered (RECOVER_READY_MS after a LINK_TIMEOUT_MS
/// COMRESET) before either stops waiting.
pub const COMMAND_MS: u64 = 30_000;
