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

//! Running a tool as a job: one staged in the vfs store, or one baked in.

use crate::command::builtin::nox::install;
use crate::command::builtin::tool;
use crate::command::dispatch::hears_end_of_input;
use crate::jobs::stdin_queue::StdinQueue;
use crate::jobs::tty;
use crate::jobs::work::JobWork;
use crate::term::state::State;

use super::Verdict;

// The CLI tools staged in the vfs store. A bare invocation of one of these runs
// it from the store instead of falling through to "unknown verb".
const STORE_TOOLS: &[&[u8]] = &[b"sd", b"tokio-smoke", b"std_proof"];

/// Whether a bare name runs from the store. Read from the same list the
/// dispatcher uses, so what `type` reports is what would run.
pub fn is_store_tool(name: &[u8]) -> bool {
    STORE_TOOLS.contains(&name)
}

/// The job a tool named by `args[0]` becomes, or `Instant` for a name that
/// is no tool.
pub(super) fn tool_job(state: &mut State, args: &[&[u8]]) -> Verdict {
    match args[0] {
        // Bare-name run of a store tool staged in the vfs (`sd a b`, ...): route
        // it through the same async installer path as `install <name>`, so the
        // tool loads and streams its output without blocking the event loop.
        // The tool name is args[0], which prepare reads as the capsule stem.
        name if STORE_TOOLS.contains(&name) => match install::prepare(state, args) {
            Some(job) => Verdict::Job(JobWork::InstallDrain(job)),
            None => {
                state.last_status = 1;
                Verdict::Handled
            }
        },
        /* A baked tool, `linux` among them, runs as a foreground program for as
         * long as it runs: an install's drain gave up on it at 5 s. */
        name if tool::is_tool(name) => match tool::prepare(state, args, tty::streams(false, false))
        {
            Some(pid) => {
                /* The Linux personality hears a lone 0x04 as end of input. */
                let stdin = StdinQueue::new(hears_end_of_input(name));
                Verdict::Job(JobWork::ExternalStage { pid, stdin, capture: None })
            }
            None => Verdict::Handled,
        },
        _ => Verdict::Instant,
    }
}
