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

//! `install` as a job: a short wait for the installer, then the load followed from ticks.

use nonos_libc::mk_time_millis;

use super::super::call::{issue, Issued};
use super::super::children::children;
use super::super::emit::{emit_err, emit_ok};
use super::super::follow::Follow;
use super::super::job::InstallJob;
use super::{argv_blob, debug_marker, valid_name};
use crate::command::output::Output;
use crate::term::state::State;

// Job-submission variant of `run`: the same request and success/failure
// reporting, but the installer is waited on only briefly, a load it is still
// running is followed from the job's ticks, and the stdout drain is the
// job's too, instead of either holding the window.
pub fn prepare(state: &mut State, args: &[&[u8]]) -> Option<InstallJob> {
    if args.is_empty() {
        state.scrollback.push_error(b"usage: install <name> [argv...]");
        return None;
    }
    let stem = args[0];
    if !valid_name(stem) {
        state.scrollback.push_error(b"install: name must be ascii letters, digits, _ or -");
        return None;
    }
    let argv = argv_blob(stem, &args[1..]);
    let before = children();
    let mut out = Output::new(&mut state.scrollback);
    match issue(stem, &argv) {
        Issued::Loaded(new_pid) => {
            emit_ok(&mut out, stem, new_pid);
            debug_marker(b"[TERMINAL-INSTALL] load ok\n");
            Some(InstallJob::new(new_pid))
        }
        Issued::Refused(status) => {
            emit_err(&mut out, status);
            debug_marker(b"[TERMINAL-INSTALL] load failed\n");
            None
        }
        Issued::InFlight => {
            let mut line = alloc::vec::Vec::from(&b"loading "[..]);
            line.extend_from_slice(stem);
            line.extend_from_slice(b"...");
            out.writeln(&line);
            Some(InstallJob::loading(stem, Follow::new(before, mk_time_millis())))
        }
    }
}
