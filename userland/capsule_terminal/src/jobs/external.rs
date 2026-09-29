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

use nonos_libc::{mk_proc_output, mk_wait};

use super::external_io::{drain_remaining, TICK_BUDGET};
use super::stdin_queue::StdinQueue;
use crate::command::output::Output;
use crate::jobs::JobProgress;

const ERRNO_TIMEDOUT: i64 = -110;
pub fn step_external(pid: u32, stdin: &mut StdinQueue, out: &mut Output<'_>) -> JobProgress {
    stdin.feed(pid);
    let mut buf = [0u8; 256];
    let mut taken = 0;
    while taken < TICK_BUDGET {
        let n = mk_proc_output(pid, buf.as_mut_ptr(), buf.len());
        if n <= 0 {
            break;
        }
        let n = (n as usize).min(buf.len());
        out.feed_raw(&buf[..n]);
        taken += n;
    }
    /*
     * What the program asked the terminal (its cursor position, its
     * identity) is answered on its stdin, as a tty answers.
     */
    let replies = out.take_replies();
    if !replies.is_empty() {
        let _ = stdin.push(&replies);
    }
    let status = mk_wait(pid as u64, 0);
    if status == ERRNO_TIMEDOUT {
        return JobProgress::Running;
    }
    drain_remaining(pid, out, &mut buf);
    out.program_ended();
    JobProgress::Done(status as i32)
}
