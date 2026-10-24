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

//! How long to wait after a directory sweep that reached no authority, as a
//! pure rule. Before the link is up every authority is refused at once, and
//! a sweep a second would ask seven times a second for nothing. The wait
//! doubles from one second to half a minute, and is said only when it grows,
//! so a boot without a network says so a handful of times, not every second.
//!
//! No crate paths here, so the host proofs mount this file as it is.

/// The wait after the first empty sweep, and the most any wait grows to. The
/// manager's clock is in whole seconds (server::runner::seconds), so these
/// are seconds, not milliseconds.
pub const FIRST_SECONDS: u64 = 1;
pub const MOST_SECONDS: u64 = 30;

/// The wait after the `empty`th empty sweep in a row, counted from 1.
pub fn seconds(empty: u32) -> u64 {
    let doublings = empty.saturating_sub(1).min(16);
    (FIRST_SECONDS << doublings).min(MOST_SECONDS)
}

/// Whether the `empty`th empty sweep in a row is worth a line: the first,
/// and each one whose wait is longer than the one before.
pub fn worth_saying(empty: u32) -> bool {
    empty == 1 || seconds(empty) > seconds(empty.saturating_sub(1))
}
