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

use crate::term::state::State;

use super::pipeline_job::PipelineJob;
use super::pipeline_stages::{is_plain, pipeline_stages};
use super::work::JobWork;

#[path = "classify_builtin.rs"]
mod classify_builtin;
#[path = "classify_network.rs"]
mod classify_network;
#[path = "classify_package.rs"]
mod classify_package;
#[path = "classify_tool.rs"]
mod classify_tool;
use classify_builtin::builtin_job;
use classify_network::network;
use classify_package::package;
pub use classify_tool::is_store_tool;

// `Instant` statements run through the untouched synchronous dispatch;
// `Job` carries the prepared work to submit; `Handled` means the command
// was job-eligible but its setup already failed and reported (a second
// synchronous run would repeat the error and its wait), so the statement
// is consumed with `state.last_status` left as the setup decided.
pub enum Verdict {
    Instant,
    Job(JobWork),
    Handled,
}

// Long-running builtins that must not block the event loop: `ping` (a
// network round trip), `install` (an installer call plus stdout drain),
// and `http` and `git clone` (`network` below). A plain invocation of one
// becomes a stepped job directly; one piped into a pipeline (`ping x | grep y`) becomes a stepped
// `PipelineStages` job instead, so it does not block behind `command::run`.
// A baked tool with a redirect runs as its job with the redirect honoured
// (tool_redirect). Any other non-plain statement (redirects, or an
// all-instant pipeline) stays on the existing synchronous dispatch.
pub fn is_job_command(state: &mut State, args: &[&[u8]]) -> Verdict {
    if args.is_empty() {
        return Verdict::Instant;
    }
    if let Some(verdict) = super::tool_redirect::redirected(state, args) {
        return verdict;
    }
    if !is_plain(args) {
        return match pipeline_stages(args) {
            Some(stages) => Verdict::Job(JobWork::PipelineStages(PipelineJob::new(stages))),
            None => Verdict::Instant,
        };
    }
    if let Some(verdict) = network(state, args) {
        return verdict;
    }
    if let Some(verdict) = package(state, args) {
        return verdict;
    }
    builtin_job(state, args)
}
