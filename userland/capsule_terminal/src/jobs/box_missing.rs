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

//! What is said when a boxed run's report never came whole: on the screen,
//! why, and on the serial log, a `[BOX-MISSING]` line, so a check of the run
//! never reads a missing report as a run that refused nothing. The `[BOX]`
//! line's own format (`v=1`) is not touched; this is a line of its own.

use alloc::format;
use alloc::string::String;

/// The status a capsule panic ends with (libc's panic handler).
pub const PANIC_EXIT: i32 = 134;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Missing {
    /// The run ended and no frame of its report came.
    None,
    /// Some of it came, not all.
    Incomplete,
    /// Ctrl+C stopped the run before all of it came.
    Stopped,
}

/// The screen's line for a report missing this way, the run ending `exit`.
pub fn said(m: Missing, exit: i32) -> &'static [u8] {
    match (m, exit) {
        (Missing::None, PANIC_EXIT) => {
            b"linux: the box sent no report: capsule_linux stopped on an internal error \
              (exit 134), so what the program tried is not known"
        }
        (Missing::None, _) => b"linux: the box sent no report; the run ended before it could",
        (Missing::Incomplete, _) => b"linux: the box's report arrived incomplete",
        (Missing::Stopped, _) => b"linux: stopped before the box's report arrived",
    }
}

/// The serial log's line for it. Every field is always present.
pub fn line(m: Missing, exit: i32) -> String {
    let report = match m {
        Missing::None => "none",
        Missing::Incomplete => "incomplete",
        Missing::Stopped => "stopped",
    };
    let failed = if m == Missing::None && exit == PANIC_EXIT { "yes" } else { "no" };
    format!("[BOX-MISSING] v=1 report={report} capsule_failed={failed} exit={exit} source=terminal")
}
