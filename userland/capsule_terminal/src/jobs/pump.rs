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

use alloc::vec::Vec;

use crate::command::output::Output;
use crate::term::state::State;

use super::pump_pipeline::step_pipeline_job;
use super::pump_shown::Seen;
use super::reap::reap;
use super::table::{JobProgress, JobState};
use super::work::{step, JobWork};

// Steps every live job one bounded slice into the real scrollback, then
// hands off to `reap`. True only when that showed something: a write that
// reached the screen, or a job that ended or went. A job that is only alive,
// a Linux app with a window of its own, asks for no repaint, which would
// draw the whole window again every tick for as long as the app runs.
pub fn pump(state: &mut State) -> bool {
    let ids: Vec<u32> =
        state.jobs.iter().filter(|j| j.state == JobState::Running).map(|j| j.id).collect();
    if ids.is_empty() {
        return false;
    }
    let before = Seen::of(state);
    for id in ids {
        step_job(state, id);
    }
    reap(state);
    Seen::of(state) != before
}

fn step_job(state: &mut State, id: u32) {
    let (cancel, held) = match state.jobs.get(id) {
        Some(job) => (job.cancel, state.fg_running),
        None => return,
    };
    if !cancel && matches!(state.jobs.get(id).map(|j| &j.work), Some(JobWork::PipelineStages(_))) {
        return step_pipeline_job(state, id);
    }
    if let Some(job) = state.jobs.get_mut(id) {
        let mut out = Output::new(&mut state.scrollback);
        if let JobProgress::Done(status) = step(job, &mut out, held) {
            job.status = status;
            job.state = JobState::Done;
        }
    }
    let _ = super::net_again::again(state, id);
}
