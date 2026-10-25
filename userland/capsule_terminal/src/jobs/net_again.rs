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


//! Starting a plain `linux` run again with the network, once, when it
//! ended asking for it (net_ask.rs): the same job, block and streams, a new
//! pid, and its input handed to it again from the start.

use nonos_libc::{mk_tool_run, LINUX_ASK_NET};

use super::net_ask::{ended, NET_TOOL};
use super::table::JobState;
use super::work::JobWork;
use crate::term::state::State;

/// Whether the job `id`, just done, was the run that asked and now runs again.
pub(super) fn again(state: &mut State, id: u32) -> bool {
    let (pid, status) = match state.jobs.get(id) {
        Some(j) if j.state == JobState::Done => match j.work {
            JobWork::ExternalStage { pid, .. } => (pid, j.status),
            _ => return false,
        },
        _ => return false,
    };
    let Some(ask) = ended(&mut state.jobs.net_ask, pid, status, LINUX_ASK_NET) else {
        return false;
    };
    let rc = mk_tool_run(NET_TOOL, &ask.argv);
    if rc < 0 {
        crate::command::builtin::tool_refused::refused(state, b"linux", rc);
        return false;
    }
    super::tty::attach_streams(state, rc as u32, ask.streams);
    let Some(job) = state.jobs.get_mut(id) else { return false };
    if let JobWork::ExternalStage { pid, stdin, .. } = &mut job.work {
        *pid = rc as u32;
        stdin.again();
    }
    job.status = 0;
    job.state = JobState::Running;
    true
}
