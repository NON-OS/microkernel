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

//! A `pkg` job looking for its worker's answer each tick, and letting go of it on Ctrl+C.

use nonos_libc::Look;

use super::super::work::report;
use super::{PkgJob, ANSWER};
use crate::command::output::Output;
use crate::jobs::JobProgress;

impl PkgJob {
    pub fn step_once(&mut self, out: &mut Output<'_>) -> JobProgress {
        match ANSWER.poll() {
            Look::Waiting => JobProgress::Running,
            Look::Ready(done) => {
                self.answered = true;
                JobProgress::Done(if report(out, &done) { 0 } else { 1 })
            }
            Look::Idle => {
                self.answered = true;
                out.writeln_error(b"pkg: the installer's answer was lost");
                JobProgress::Done(1)
            }
        }
    }
}

impl Drop for PkgJob {
    /// Ctrl+C ends the job without its answer: stop waiting for it, so the
    /// worker's answer is discarded when it comes rather than kept.
    fn drop(&mut self) {
        if !self.answered {
            ANSWER.stop_waiting();
        }
    }
}
