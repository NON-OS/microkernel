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

use alloc::boxed::Box;

use crate::command::builtin::git::clone::CloneJob;
use crate::command::builtin::nox::http::HttpJob;
use crate::command::builtin::nox::install::InstallJob;
use crate::command::builtin::nox::pkg::PkgJob;
use crate::command::builtin::ping::{emit_probe, PingJob};
use crate::command::output::Output;

use super::capture::Capture;
use super::pipeline_job::PipelineJob;
use super::stdin_queue::StdinQueue;
use super::table::{JobProgress, JobRecord};

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
    /* A program the terminal started. `capture` holds its output when it
     * goes to a file or to /dev/null rather than the screen. */
    ExternalStage { pid: u32, stdin: StdinQueue, capture: Option<Box<Capture>> },
    /* A request over the network, and a clone, carried a tick at a time;
     * dropped on Ctrl+C, which closes their connection. */
    Http(Box<HttpJob>),
    GitClone(Box<CloneJob>),
    /* An installer call waiting on a worker thread; on Ctrl+C the worker
     * finishes and its answer is dropped. */
    Pkg(Box<PkgJob>),
}

// Step a job's work by one bounded slice. A cancelled job is finished
// unconditionally, regardless of variant: the terminal reports it as
// interrupted rather than letting the underlying poll run to completion.
pub fn step(job: &mut JobRecord, out: &mut Output<'_>, held: bool) -> JobProgress {
    let leave_modes = job.background && held;
    if job.cancel {
        /* A program stopped by Ctrl+C never gets to undo the screen modes it
         * set, the alternate screen, a hidden cursor, a colour left on: it
         * ends here the way a program that exits on its own ends. What it
         * wrote after the key is dropped, as a tty drops it on an interrupt. */
        if let JobWork::ExternalStage { pid, ref capture, .. } = job.work {
            super::external_io::discard_output(pid);
            if !leave_modes {
                out.program_ended();
            }
            if let Some(capture) = capture {
                capture.interrupted(out);
            }
        }
        out.writeln(b"interrupted");
        return JobProgress::Done(130);
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
        JobWork::ExternalStage { pid, stdin, capture } => {
            super::external::step_external(*pid, stdin, capture.as_deref_mut(), out, leave_modes)
        }
        JobWork::Http(job) => job.step_once(out),
        JobWork::GitClone(job) => job.step_once(out),
        JobWork::Pkg(job) => job.step_once(out),
        JobWork::PipelineStages(_) => JobProgress::Running,
    }
}
