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

//! How often the Terminal asks the shell for a handed command. A tool tile
//! that launched the Terminal left the command before the window existed, so
//! for its first seconds the window asks on every tick (30 ms) and the
//! command appears with the window. After that a command can only come from a
//! tile clicked while the Terminal is open, which the shell answers by
//! raising the window; asking twice a second shows it about as soon as the
//! window comes up, and costs the shell two short calls a second.

/// How long after start the window asks on every tick.
pub const STARTUP_MS: i64 = 3_000;
/// How often it asks after that.
pub const EVERY_MS: i64 = 500;

/// When to ask next, having just asked at `now` in a window that started at
/// `started`.
pub fn next_ask(now: i64, started: i64) -> i64 {
    if now.saturating_sub(started) < STARTUP_MS {
        now
    } else {
        now.saturating_add(EVERY_MS)
    }
}
