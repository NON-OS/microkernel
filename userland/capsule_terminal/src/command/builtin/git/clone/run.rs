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
//! Running a clone where it is not a job of its own: `nox git clone`, or a
//! clone in a pipeline. The same stepped job is driven to its end inline,
//! and the window waits for it, as it always did there.

use nonos_libc::mk_yield;

use super::job::prepare;
use crate::command::output::Output;
use crate::jobs::JobProgress;
use crate::term::state::State;

pub(in crate::command::builtin::git) fn run(state: &mut State, argv: &[&[u8]]) {
    let Some(mut job) = prepare(state, argv) else {
        return;
    };
    let mut out = Output::new(&mut state.scrollback);
    while job.step_once(&mut out) == JobProgress::Running {
        mk_yield();
    }
}
