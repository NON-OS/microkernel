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

//! The long-running builtins and tools a plain statement names, made jobs.

use crate::command::builtin::nox::install;
use crate::command::builtin::ping;
use crate::command::builtin::qwen;
use crate::command::output::Output;
use crate::jobs::work::JobWork;
use crate::term::state::State;

use super::classify_tool::tool_job;
use super::Verdict;

/// The job a plain statement starting with `args[0]` becomes, or `Instant`
/// for one the synchronous dispatch runs.
pub(super) fn builtin_job(state: &mut State, args: &[&[u8]]) -> Verdict {
    match args[0] {
        b"ping" => {
            let mut out = Output::new(&mut state.scrollback);
            match ping::prepare(&mut out, args) {
                Some(job) => Verdict::Job(JobWork::Ping(job)),
                None => Verdict::Handled,
            }
        }
        b"install" => match install::prepare(state, &args[1..]) {
            Some(job) => Verdict::Job(JobWork::InstallDrain(job)),
            None => {
                state.last_status = 1;
                Verdict::Handled
            }
        },
        /*
         * `qwen` after a `;`, `&&` or `||`, or from an alias: the chat starts
         * with no question, since the parser has already taken the line apart.
         */
        b"qwen" => match qwen::from_args(state, args) {
            Some(work) => Verdict::Job(work),
            None => Verdict::Handled,
        },
        _ => tool_job(state, args),
    }
}
