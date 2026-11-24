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

//! Starting what a queued run asked for, said on the log.

use crate::sys::serial::Line;
use crate::userspace::capsule_linux::{run_started, spawn_run};

/// How a queued run starts.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum How {
    /// The run role, said with its package.
    Named,
    /// The run role, said without its package: a shipped tier a terminal
    /// asked for is named nowhere on the log.
    Quiet,
}

pub(super) fn run(package: &str, how: How) {
    // The process a capsule's launch is held by (capsule_linux/served_run.rs).
    let started = || spawn_run(package).map(|pid| run_started(package, pid)).is_ok();
    let said: &[u8] = match super::super::app_choice::linux_off() {
        true => b"[LINUX-RUN] Linux turned off at setup, not started: ",
        false if started() => b"[LINUX-RUN] started ",
        false => b"[LINUX-RUN] refused ",
    };
    let mut l = Line::new();
    match how {
        How::Named => l.str(said).app(package.as_bytes()),
        How::Quiet => l.str(said).str(b"a Qwen window"),
    };
    l.end();
}
