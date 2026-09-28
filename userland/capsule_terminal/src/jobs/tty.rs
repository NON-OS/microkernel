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

//! Telling a program that its output reaches this screen, and how big the
//! screen is. The kernel answers the program's own is-it-a-terminal
//! question from this, so it picks colour and columns for a person. A
//! stage that feeds a pipe or a file is never told, and writes plain bytes.

use nonos_libc::{mk_tty_set, TTY_STDERR, TTY_STDIN, TTY_STDOUT};

use super::JobWork;
use crate::term::state::State;

/// The program `pid` reads the keyboard and writes to this screen. A kernel
/// without the call leaves it believing it writes to a pipe, which is where
/// it started, so the answer is not checked.
pub fn attach(state: &State, pid: u32) {
    let vt = &state.scrollback.vt;
    let (cols, rows) = (vt.cols().min(u16::MAX as usize), vt.rows().min(u16::MAX as usize));
    let _ = mk_tty_set(pid, TTY_STDIN | TTY_STDOUT | TTY_STDERR, cols as u16, rows as u16);
}

/// The screen changed size: the program in front hears the new one the
/// next time it asks.
pub fn resized(state: &State) {
    let Some(id) = state.jobs.foreground() else { return };
    let pid = match state.jobs.get(id).map(|j| &j.work) {
        Some(JobWork::ExternalStage { pid, .. }) => *pid,
        Some(JobWork::InstallDrain(job)) => job.pid,
        _ => return,
    };
    attach(state, pid);
}
