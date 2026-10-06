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
    attach_streams(state, pid, ALL);
}

/// Every standard stream on this screen.
const ALL: u64 = TTY_STDIN | TTY_STDOUT | TTY_STDERR;

/*
 * A stream redirected to or from a file (`> f`, `< f`) is not this screen,
 * so it is left out and the program, asking, writes plain bytes there and
 * reads its input as a file. stderr stays on the screen: an error is still
 * the person's to read. The input `< f` gives still comes over this
 * terminal's channel; the Linux personality reads it while any of its
 * streams is attached, and tags its output when stdout is not (TAG_STDOUT).
 */
/// The streams to attach for a program whose output goes to a file or not,
/// and whose input comes from one or not.
pub const fn streams(output_redirected: bool, input_redirected: bool) -> u64 {
    let mut on = TTY_STDERR;
    if !output_redirected {
        on |= TTY_STDOUT;
    }
    if !input_redirected {
        on |= TTY_STDIN;
    }
    on
}

/// `attach` for the streams named, as `streams` gives them.
pub fn attach_streams(state: &State, pid: u32, streams: u64) {
    let vt = &state.scrollback.vt;
    let (cols, rows) = (vt.cols().min(u16::MAX as usize), vt.rows().min(u16::MAX as usize));
    let _ = mk_tty_set(pid, streams, cols as u16, rows as u16);
}

/// The screen changed size: the program in front hears the new one the
/// next time it asks.
pub fn resized(state: &State) {
    let Some(id) = state.jobs.foreground() else { return };
    let (pid, streams) = match state.jobs.get(id).map(|j| &j.work) {
        Some(JobWork::ExternalStage { pid, capture, stdin }) => {
            (*pid, streams(capture.is_some(), !stdin.takes_keys()))
        }
        Some(JobWork::InstallDrain(job)) if job.pid != 0 => (job.pid, ALL),
        _ => return,
    };
    attach_streams(state, pid, streams);
}
