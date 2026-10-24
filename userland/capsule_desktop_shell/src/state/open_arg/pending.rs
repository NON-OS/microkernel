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

//! A command waiting for the Terminal, and a path told from it.

use alloc::string::String;

/// The only app a command is handed to.
pub const TERMINAL: &[u8] = b"app.terminal";

/// How long a command waits for the Terminal. A tile launches the Terminal
/// and its window asks within a few ticks of starting; a command still there
/// long after was for a window that never came up, and must not run in the
/// next Terminal someone opens for something else.
pub const COMMAND_TTL_MS: i64 = 10_000;

/// A command waiting for the Terminal to ask.
pub struct PendingCommand {
    pub line: String,
    /// When it was left, on uptime: the wall clock reads an error before the
    /// RTC is read and NTP steps it back, which made a command left then
    /// stale at once, or kept it for hours.
    pub at_ms: i64,
}

/// Whether `arg` may be left for an app as a path.
pub fn is_path(arg: &str) -> bool {
    arg.starts_with('/')
}

/// Whether a command left at `at_ms` may still be handed over at `now`.
pub fn still_wanted(at_ms: i64, now: i64) -> bool {
    (0..COMMAND_TTL_MS).contains(&now.saturating_sub(at_ms))
}
