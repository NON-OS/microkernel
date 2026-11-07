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

//! Ticks against the uptime clock. The cycle counter's rate is not handed
//! to a capsule, so it is measured: ticks over milliseconds since tracing
//! began, once enough milliseconds passed that one of rounding is small.

/// Milliseconds before the rate is trusted: 1 ms of rounding is then 1%.
pub const SETTLE_MS: u64 = 100;

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Calib {
    pub ticks0: u64,
    pub ms0: u64,
}

impl Calib {
    /// Ticks per millisecond, once settled.
    pub fn per_ms(&self, ticks: u64, ms: u64) -> Option<u64> {
        let spent = ms.checked_sub(self.ms0).filter(|&d| d >= SETTLE_MS)?;
        Some(ticks.checked_sub(self.ticks0)? / spent).filter(|&r| r > 0)
    }
}
