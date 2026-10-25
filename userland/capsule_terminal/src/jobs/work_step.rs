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

//! Stepping a job's work by one bounded slice, or ending it on Ctrl+C.

use crate::command::builtin::ping::emit_probe;
use crate::command::output::Output;

use super::JobWork;
use crate::jobs::table::{JobProgress, JobRecord};

// Step a job's work by one bounded slice. A cancelled job is finished
// unconditionally, regardless of variant: the terminal reports it as
// interrupted rather than letting the underlying poll run to completion.
pub fn step(job: &mut JobRecord, out: &mut Output<'_>, held: bool) -> JobProgress {
    let leave_modes = job.background && held;
    if job.cancel {
        if let JobWork::ExternalStage { pid, ref capture, ref mut boxed, .. } = job.work {
            let (capture, boxed) = (capture.as_deref(), boxed.as_deref_mut());
            return super::super::external_stop::stop_external(
                pid,
                capture,
                boxed,
                out,
                leave_modes,
            );
        }
        out.writeln(b"interrupted");
        return JobProgress::Done(super::super::box_wait::STOPPED);
    }
    match &mut job.work {
        JobWork::Noop => JobProgress::Done(0),
        JobWork::Ping(job) => match job.step_once() {
            None => JobProgress::Running,
            Some(probe) => {
                let dst = job.dst();
                JobProgress::Done(emit_probe(out, dst, probe))
            }
        },
        JobWork::InstallDrain(job) => job.step_once(out),
        JobWork::ExternalStage { pid, stdin, capture, boxed } => {
            super::super::external::step_external(
                *pid,
                stdin,
                capture.as_deref_mut(),
                boxed.as_deref_mut(),
                out,
                leave_modes,
            )
        }
        JobWork::Http(job) => job.step_once(out),
        JobWork::GitClone(job) => job.step_once(out),
        JobWork::Pkg(job) => job.step_once(out),
        JobWork::PipelineStages(_) => JobProgress::Running,
    }
}
