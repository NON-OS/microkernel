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

use super::pipeline_job::step_pipeline;
use super::table::{JobProgress, JobState};
use super::work::JobWork;

// `PipelineStages` needs `&mut State` (to run non-filter stages through
// `exec`), which `work::step` does not take. The work is moved out of the
// job record so `state` is fully free for `step_pipeline`, then moved back
// in with the resulting status applied.
pub(super) fn step_pipeline_job(state: &mut State, id: u32) {
    let mut work = match state.jobs.get_mut(id) {
        Some(job) => core::mem::replace(&mut job.work, JobWork::Noop),
        None => return,
    };
    let progress = match &mut work {
        JobWork::PipelineStages(pj) => step_pipeline(pj, state),
        _ => JobProgress::Running,
    };
    if let Some(job) = state.jobs.get_mut(id) {
        job.work = work;
        if let JobProgress::Done(status) = progress {
            job.status = status;
            job.state = JobState::Done;
        }
    }
}
