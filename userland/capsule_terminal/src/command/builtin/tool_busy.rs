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

//! A tool the kernel says is already running, when it may only be ending.
//! Pure, so terminal_line_proofs holds it.
//!
//! The terminal finishes a job when `mk_wait` gives the program's status,
//! and the kernel frees the program's endpoints a little later, when its
//! teardown runs. `linux` runs under one endpoint (`app.linux.term`), so a
//! second `linux` asked for in between was refused as "one is already
//! running" though the first had ended: the second Linux command after the
//! first did not open. A refusal of `linux` is now asked again for a short
//! while before it is said, which a teardown is over well within.

pub const ERRNO_EXIST: i64 = -17;

/// How long a refused `linux` is asked again for, and how far apart.
pub const RETRY_MS: u64 = 1_000;
pub const GAP_MS: u64 = 20;

/// Whether a refusal `rc` of tool `name` may be the last run still being
/// torn down, and is asked again.
pub fn may_be_ending(name: &[u8], rc: i64) -> bool {
    name == b"linux" && rc == ERRNO_EXIST
}

/// What follows the tool's name when the kernel still refuses it as running.
pub fn running_said(name: &[u8]) -> &'static [u8] {
    match name {
        b"linux" => b": a Linux program is already running in a terminal; one runs at a time, so end it (Ctrl-C) first",
        _ => b": one is already running",
    }
}
