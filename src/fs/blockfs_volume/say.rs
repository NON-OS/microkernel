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
//! The volume's own lines. Info-level log lines stay in the RAM log, off
//! the console; what the data volume did, and the digest of each model it
//! holds, is what an operator checks first, so these reach the console too.

use alloc::format;

/// Say `line` on the console and record it in the log.
pub(super) fn say(line: &str) {
    crate::sys::serial::println(line.as_bytes());
    crate::log::info!("{}", line);
}

/// Each time an import passes a 64 MiB mark, how far it has come: a large
/// model takes minutes to seal, and a silent minute reads as a hang.
pub(super) fn progress(before: u64, after: u64, total: u64) {
    if before >> 26 != after >> 26 || after == total {
        say(&format!("[DATA] import: {} of {} MiB sealed", after >> 20, total >> 20));
    }
}
