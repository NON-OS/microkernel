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

//! A wait's time budget on a clock that can fail. A failed read spends the
//! budget at once: a wait begun on a good read whose later reads fail must
//! still end, and one begun on a failed read has no start to measure from.
//! Pure, so the host proofs hold it.

#[derive(Clone, Copy, Debug)]
pub struct Budget {
    start: Option<u64>,
    limit_ms: u64,
}

impl Budget {
    /// A budget of `limit_ms` starting at the clock read `now`.
    pub const fn begin(now: Option<u64>, limit_ms: u64) -> Self {
        Self { start: now, limit_ms }
    }

    /// Milliseconds left at the clock read `now`: none once `limit_ms` have
    /// passed or either read failed. A clock that went backwards counts as
    /// no time passed.
    pub const fn left(&self, now: Option<u64>) -> u64 {
        match (self.start, now) {
            (Some(start), Some(now)) => self.limit_ms.saturating_sub(now.saturating_sub(start)),
            _ => 0,
        }
    }
}
