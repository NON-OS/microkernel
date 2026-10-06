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

//! How long a SWITCH may keep the card busy.

use super::offsets::{GENERIC_CMD6_TIME, PARTITION_CONFIG, PARTITION_SWITCH_TIME};
use super::register::ExtCsd;

/// A SWITCH waits at least this long (Linux's DEFAULT_CMD6_TIMEOUT_MS)...
pub const CMD6_MIN_MS: u64 = 500;
/// ...and no longer than this, inside the kernel's five-second reply wait.
pub const CMD6_MAX_MS: u64 = 3_000;

impl ExtCsd {
    /// How long a SWITCH of `index` may keep the card busy, in ms:
    /// PARTITION_SWITCH_TIME for PARTITION_CONFIG, GENERIC_CMD6_TIME for
    /// the rest (both in 10 ms units), held to [CMD6_MIN_MS, CMD6_MAX_MS].
    pub const fn switch_ms(&self, index: u8) -> u64 {
        let units = if index == PARTITION_CONFIG {
            self.raw[PARTITION_SWITCH_TIME]
        } else {
            self.raw[GENERIC_CMD6_TIME]
        };
        let ms = units as u64 * 10;
        if ms < CMD6_MIN_MS {
            CMD6_MIN_MS
        } else if ms > CMD6_MAX_MS {
            CMD6_MAX_MS
        } else {
            ms
        }
    }
}
