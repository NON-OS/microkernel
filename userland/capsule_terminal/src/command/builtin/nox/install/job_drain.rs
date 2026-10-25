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

//! The drain side of an install job: the loaded capsule's stdout into the window, a read a slice.

use nonos_libc::{mk_pid_alive, mk_proc_output, mk_time_millis, mk_yield};

use super::super::run::debug_marker;
use super::InstallJob;
use crate::command::output::Output;
use crate::jobs::JobProgress;

const DEADLINE_MS: i64 = 5000;

impl InstallJob {
    pub fn step_once(&mut self, out: &mut Output<'_>) -> JobProgress {
        if self.load.is_some() {
            return self.follow(out);
        }
        // Once, after the load: a program that draws opens its window and runs on.
        if !core::mem::replace(&mut self.asked_window, true) && super::super::window::open(self.pid)
        {
            out.writeln(b"it runs in its own window");
            return JobProgress::Done(0);
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

    fn mark_output_drained(&mut self) {
        if self.saw_output {
            return;
        }
        self.saw_output = true;
        debug_marker(b"[TERMINAL-MOUT] output drained\n");
    }
}
