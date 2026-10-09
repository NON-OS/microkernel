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

//! How soon an empty library is listed again.
//!
//! An empty library is listed again from every paint and tick, up to 24
//! times, for a store still filling early in a boot. A listing the store
//! did not answer waits out the whole reply timeout (five seconds) on the
//! window's thread, so 24 of them held the window for two minutes. After a
//! silent listing the next waits this long; one the store answered, empty
//! or refused, may follow at once.

/// What the vfs client says when the store did not answer.
pub const SILENT: &str = "vfs ipc failed";

/// The least time after a silent listing before the next.
pub const SILENT_GAP_MS: i64 = 10_000;

/// When the next listing may run, the last having ended at `now_ms` with
/// `error`.
pub fn next_rescan_ms(error: Option<&str>, now_ms: i64) -> i64 {
    match error {
        Some(SILENT) => now_ms.saturating_add(SILENT_GAP_MS),
        _ => now_ms,
    }
}
