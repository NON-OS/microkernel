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

use nonos_libc::{mk_pid_alive, mk_proc_output, mk_time_millis, mk_yield};

use super::children::new_child;
use super::emit::emit_ok;
use super::follow::{Follow, Step};
use super::probe::probe;
use super::run::debug_marker;
use crate::command::output::Output;
use crate::jobs::JobProgress;

const DEADLINE_MS: i64 = 5000;

// One bounded slice of an install: while the installer is still loading,
// one look for the program it loads (`follow.rs`); then draining the loaded
// capsule's stdout into the terminal window, one `mk_proc_output` read, or,
// once the child has exited, the final flush of whatever is still buffered.
// Holds the progress cursor (elapsed start, whether any output has been
// seen) between slices.
pub struct InstallJob {
    /// Zero while the load is still being followed.
    pub(crate) pid: u32,
    stem: Vec<u8>,
    load: Option<Follow>,
    start: i64,
    saw_output: bool,
}

impl InstallJob {
    pub fn new(pid: u32) -> Self {
        Self { pid, stem: Vec::new(), load: None, start: mk_time_millis(), saw_output: false }
    }

    /// A load the installer had not answered within the issue's wait.
    pub(super) fn loading(stem: &[u8], follow: Follow) -> Self {
        Self {
            pid: 0,
            stem: stem.to_vec(),
            load: Some(follow),
            start: mk_time_millis(),
            saw_output: false,
        }
    }

    pub fn step_once(&mut self, out: &mut Output<'_>) -> JobProgress {
        if self.load.is_some() {
            return self.follow(out);
        }
        let mut buf = [0u8; 256];
        let n = mk_proc_output(self.pid, buf.as_mut_ptr(), buf.len());
        if n > 0 {
            out.feed_raw(&buf[..(n as usize).min(buf.len())]);
            self.mark_output_drained();
            return JobProgress::Running;
        }
        if !mk_pid_alive(self.pid) {
            loop {
                let m = mk_proc_output(self.pid, buf.as_mut_ptr(), buf.len());
                if m <= 0 {
                    break;
                }
                out.feed_raw(&buf[..(m as usize).min(buf.len())]);
                self.mark_output_drained();
            }
            return JobProgress::Done(0);
        }
        if mk_time_millis().wrapping_sub(self.start) > DEADLINE_MS {
            return JobProgress::Done(1);
        }
        mk_yield();
        JobProgress::Running
    }

    fn follow(&mut self, out: &mut Output<'_>) -> JobProgress {
        let now = mk_time_millis();
        let Some(follow) = self.load.as_mut() else {
            return JobProgress::Running;
        };
        if !follow.due(now) {
            return JobProgress::Running;
        }
        let mut step = follow.step(now, new_child(&follow.before, follow.elapsed_ms(now)));
        if step == Step::Probe {
            follow.probed(now, probe());
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

    fn mark_output_drained(&mut self) {
        if self.saw_output {
            return;
        }
        self.saw_output = true;
        debug_marker(b"[TERMINAL-MOUT] output drained\n");
    }
}
