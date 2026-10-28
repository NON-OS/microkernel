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

//! When a surface's frame callbacks are answered: at most once a frame
//! period, 30 a second. Pure, so the host proofs hold it.

/// One frame at 30 a second.
pub const PERIOD_MS: u64 = 33;

/// When callbacks owed at `now` may be answered, the surface's last answer
/// having gone at `last`: at once for a surface idle a period or more, or
/// never answered, else a period after its last.
pub fn due(last: Option<u64>, now: u64) -> u64 {
    last.map_or(now, |l| l.saturating_add(PERIOD_MS).max(now))
}

/// How long until the first surface that holds callbacks is due, each
/// given by its last answer; None when none holds any.
pub fn next(holding: impl Iterator<Item = Option<u64>>, now: u64) -> Option<u64> {
    holding.map(|last| due(last, now) - now).min()
}
