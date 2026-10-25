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

use super::call::call_installer;
use super::emit::{emit_err, emit_ok};
use crate::command::output::Output;
use crate::term::state::State;

#[path = "run_args.rs"]
mod run_args;
#[path = "run_drain.rs"]
mod run_drain;
#[path = "run_prepare.rs"]
mod run_prepare;

use run_args::{argv_blob, valid_name};
use run_drain::drain_output;
pub use run_prepare::prepare;

// install <name> [argv...]: ask the installer to verify, load, and spawn the
// capsule held at /capsules/<name>.*. The name must be a plain ascii token
// with no path separators; the installer reads the artifacts from the store.
pub fn run(state: &mut State, args: &[&[u8]]) -> bool {
    if args.is_empty() {
        state.scrollback.push_error(b"usage: install <name> [argv...]");
        return false;
    }
    let stem = args[0];
    if !valid_name(stem) {
        state.scrollback.push_error(b"install: name must be ascii letters, digits, _ or -");
        return false;
    }
    let argv = argv_blob(stem, &args[1..]);
    match call_installer(stem, &argv) {
        Ok(new_pid) => {
            crate::jobs::tty::attach(state, new_pid);
            emit_ok(&mut Output::new(&mut state.scrollback), stem, new_pid);
            debug_marker(b"[TERMINAL-INSTALL] load ok\n");
            drain_output(state, new_pid);
            true
        }
        Err(status) => {
            emit_err(&mut Output::new(&mut state.scrollback), status);
            debug_marker(b"[TERMINAL-INSTALL] load failed\n");
            false
        }
    }
}

pub(super) fn debug_marker(msg: &[u8]) {
    let _ = nonos_libc::mk_debug(msg.as_ptr(), msg.len());
}
