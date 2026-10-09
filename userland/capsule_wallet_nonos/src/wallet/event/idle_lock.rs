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

//! When the window locks by itself: after `IDLE_LOCK_MS` with no click and no
//! key, so a wallet left open is not left readable. Never while a payment
//! is going out, which finishes and is said when the window opens. Pure, so
//! wallet_proofs holds the rule.

/// Five minutes without a press.
pub const IDLE_LOCK_MS: i64 = 5 * 60 * 1000;

/// Whether to lock now, at `now`, with the last press at `last`.
pub fn due(now: i64, last: i64, locked: bool, sending: bool) -> bool {
    !locked && !sending && now.saturating_sub(last) >= IDLE_LOCK_MS
}
