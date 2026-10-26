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

//! The inboxes a capsule owns by its pid, each under its own name, so what
//! it prints never lands where its requests are read: a server receiving on
//! `proc.<pid>` would take its own lines for requests from itself, and a
//! parent draining its output would take the requests.

extern crate alloc;

use alloc::format;
use alloc::string::String;

/// Where requests and replies addressed to `pid` arrive (MkIpcRecvFrom 0).
pub fn requests(pid: u32) -> String {
    format!("proc.{}", pid)
}

/// What its parent feeds it (MkProcInput), read with MkStdinRead.
pub fn stdin(pid: u32) -> String {
    format!("stdin.{}", pid)
}

/// What it prints (MkDebug, MkStdoutWrite, MkPrivateWrite), drained by the
/// parent that launched it (MkProcOutput).
pub fn stdout(pid: u32) -> String {
    format!("stdout.{}", pid)
}
