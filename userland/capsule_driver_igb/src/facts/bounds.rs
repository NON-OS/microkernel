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

//! Every wait, in milliseconds. Where a value was seen in another driver it
//! says so and is unverified; where it is this driver's own, it says why.
//! The error lines that quote a bound are held to it in igb_proofs.

/// Bus mastering must stop before reset: 80 ms as observed,
/// unverified.
pub const MASTER_DISABLE_MS: u64 = 80;
/// Settle after the engines stop and before reset: 10 ms as observed,
/// unverified.
pub const QUIESCE_SETTLE_MS: u64 = 10;
/// The NVM read after reset: 20 ms as observed, unverified.
pub const AUTO_READ_MS: u64 = 20;
/// Each half of the SWSM semaphore. This driver's: two seconds, past the
/// longest NVM operation firmware may hold it for (as observed,
/// a 32K-word NVM at 50 us a word, unverified).
pub const SEMAPHORE_MS: u64 = 2000;
/// The PHY bit of SW_FW_SYNC, retried every 5 ms: one second as observed,
/// unverified.
pub const SWFW_SYNC_MS: u64 = 1000;
pub const SWFW_RETRY_MS: u64 = 5;
/// One MDIC cycle: 100 ms as observed, unverified.
pub const MDIC_MS: u64 = 100;
/// A queue enable reading back. This driver's own: other drivers do not
/// read it back; 100 ms is allowed for it to show.
pub const QUEUE_ENABLE_MS: u64 = 100;
