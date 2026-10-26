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

//! A loaded capsule's stdout drained into the window on its own thread, bounded.

use super::super::job::InstallJob;
use crate::command::output::Output;
use crate::jobs::JobProgress;
use crate::term::state::State;

// Pull the loaded capsule's stdout out of its stdout.<pid> inbox and into
// the terminal window. The kernel mirrors each MkDebug line there. We
// step the drain job on the current thread until it finishes or times
// out, so a child that never exits cannot freeze the terminal.
pub(super) fn drain_output(state: &mut State, pid: u32) {
    let mut out = Output::new(&mut state.scrollback);
    let mut job = InstallJob::new(pid);
    loop {
        if let JobProgress::Done(_) = job.step_once(&mut out) {
            break;
        }
    }
}
