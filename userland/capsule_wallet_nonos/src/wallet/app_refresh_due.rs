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
//! When an account refresh may begin. Pure, so wallet_proofs checks the rule
//! against these same constants.

// Time between account refreshes, counted from the end of the last one. One
// batched fetch fills every field, so the wallet does not need to poll often;
// a refresh every dozen seconds keeps balances current.
pub const REFRESH_MS: i64 = 12_000;

// How long after the last click or keypress a refresh may begin.
pub const IDLE_MS: i64 = 2_000;

/// Whether a refresh is due at `now`: the screen shows live data, the user
/// has paused since `last_active`, and one is `owed` (none has run yet, or
/// the status step asks again) or the last ran `REFRESH_MS` ago.
pub fn due(live: bool, owed: bool, now: i64, last_active: i64, last_probe: i64) -> bool {
    live && now.wrapping_sub(last_active) >= IDLE_MS
        && (owed || now.wrapping_sub(last_probe) >= REFRESH_MS)
}
