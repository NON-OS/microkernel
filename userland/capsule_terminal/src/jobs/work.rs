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

use crate::command::builtin::nox::install::InstallJob;
use crate::command::builtin::ping::{emit_probe, PingJob};
use crate::command::output::Output;

use super::pipeline_job::PipelineJob;
use super::stdin_queue::StdinQueue;
use super::table::JobProgress;

// The step machine for long-running command kinds, one variant per kind,
// each holding the progress cursor its poll body tracks. `Noop` is the
// inert sentinel `pump` swaps in via `mem::replace(&mut job.work, Noop)`
// while it runs a `PipelineStages` job with `&mut State`. `PipelineStages`
// needs that `&mut State` to run its stages (real commands, not just
// filters), so it is stepped by `pump::step_pipeline_job` instead of this
// function; the arm below only fires if `step` is ever called on it
// directly, which pump's routing avoids for the non-cancelled case.
pub enum JobWork {
    Noop,
    Ping(PingJob),
    InstallDrain(InstallJob),
    PipelineStages(PipelineJob),
    ExternalStage { pid: u32, stdin: StdinQueue },
}

// Step a job's work by one bounded slice. A cancelled job is finished
// unconditionally, regardless of variant: the terminal reports it as
// interrupted rather than letting the underlying poll run to completion.
pub fn step(work: &mut JobWork, out: &mut Output<'_>, cancel: bool) -> JobProgress {
    if cancel {
        /*
         * A program stopped by Ctrl+C never gets to undo the screen modes it
         * set, the alternate screen, a hidden cursor, a colour left on: it
         * ends here the way a program that exits on its own ends. What it
         * wrote after the key is dropped, as a tty drops it on an interrupt.
         */
        if let JobWork::ExternalStage { pid, .. } = work {
            super::external_io::discard_output(*pid);
            out.program_ended();
        }
        out.writeln(b"interrupted");
        return JobProgress::Done(130);
    }
    match work {
        JobWork::Noop => JobProgress::Done(0),
        JobWork::Ping(job) => match job.step_once() {
            None => JobProgress::Running,
            Some(probe) => {
                let dst = job.dst();
                JobProgress::Done(emit_probe(out, dst, probe))
            }
        },
        JobWork::InstallDrain(job) => job.step_once(out),
        JobWork::ExternalStage { pid, stdin } => super::external::step_external(*pid, stdin, out),
        JobWork::PipelineStages(_) => JobProgress::Running,
    }
}
