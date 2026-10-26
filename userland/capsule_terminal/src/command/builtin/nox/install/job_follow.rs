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

//! The load side of an install job: one look a slice for the program the installer loads.

use alloc::vec::Vec;

use nonos_libc::mk_time_millis;

use super::super::children::new_child;
use super::super::emit::{emit_err, emit_ok};
use super::super::run::debug_marker;
use super::super::{follow::Step, probe::probe};
use super::InstallJob;
use crate::command::output::Output;
use crate::jobs::JobProgress;

impl InstallJob {
    pub(super) fn follow(&mut self, out: &mut Output<'_>) -> JobProgress {
        let now = mk_time_millis();
        let Some(follow) = self.load.as_mut() else {
            return JobProgress::Running;
        };
        if !follow.due(now) {
            return JobProgress::Running;
        }
        let mut step = follow.step(now, new_child(&follow.before, follow.elapsed_ms(now)));
        if step == Step::Probe {
            follow.probed(now, probe(&self.stem));
            step = follow.step(now, new_child(&follow.before, follow.elapsed_ms(now)));
        }
        match step {
            Step::Wait | Step::Probe => JobProgress::Running,
            Step::Loaded(pid) => {
                self.load = None;
                self.pid = pid;
                self.start = mk_time_millis();
                emit_ok(out, &self.stem, pid);
                debug_marker(b"[TERMINAL-INSTALL] load ok\n");
                JobProgress::Running
            }
            Step::Refused(status) => {
                self.load = None;
                emit_err(out, status);
                debug_marker(b"[TERMINAL-INSTALL] load failed\n");
                JobProgress::Done(1)
            }
            Step::Failed => self.give_up(out, b" did not load: the installer finished without it"),
            Step::TimedOut => self.give_up(out, b" did not load within 30 seconds"),
        }
    }

    fn give_up(&mut self, out: &mut Output<'_>, why: &[u8]) -> JobProgress {
        self.load = None;
        let mut line = Vec::from(&b"install: "[..]);
        line.extend_from_slice(&self.stem);
        line.extend_from_slice(why);
        out.writeln_error(&line);
        debug_marker(b"[TERMINAL-INSTALL] load failed\n");
        JobProgress::Done(1)
    }
}
