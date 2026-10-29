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

//! The line a launch says at its first window (clock.rs), in one place, so
//! a lane can read each count from it by its words. Pure.

use alloc::string::String;

/// What went into a launch up to its first window.
pub struct Counts {
    pub ms: u64,
    pub prep_ms: u64,
    pub helpers: u64,
    pub reads: u64,
    pub read_bytes: u64,
    pub calls: u64,
}

pub fn line(c: &Counts) -> String {
    alloc::format!(
        "[LINUX-RUN] window after {} ms: GTK preparation {} ms in {} runs, {} store reads \
         ({} KiB), {} calls\n",
        c.ms,
        c.prep_ms,
        c.helpers,
        c.reads,
        c.read_bytes >> 10,
        c.calls,
    )
}
