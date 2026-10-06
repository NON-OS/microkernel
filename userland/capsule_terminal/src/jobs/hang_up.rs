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

//! A tab or window going away ends the programs reading from it, as a
//! tty's hangup does. Left running, such a program has no one to take its
//! output or give it input, and keeps what it holds: a Qwen chat keeps its
//! model in memory and one of the few places the kernel has for a chat.

use nonos_libc::mk_kill;

use super::{JobState, JobWork};
use crate::term::state::State;

const SIGTERM: u64 = 15;

/// End every program this tab started that is still running, and drop what
/// it wrote that nobody will read. The tab goes either way, so a kill that
/// fails is not retried.
pub fn hang_up(state: &State) {
    for job in state.jobs.iter() {
        if job.state != JobState::Running {
            continue;
        }
        if let JobWork::ExternalStage { pid, .. } = job.work {
            let _ = mk_kill(pid as u64, SIGTERM);
            super::external_io::discard_output(pid);
        }
    }
}
