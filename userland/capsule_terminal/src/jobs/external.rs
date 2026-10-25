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
use super::{box_hold::BoxHold, capture::Capture, external_route::route, stdin_queue::StdinQueue};
use crate::command::output::Output;
use crate::jobs::JobProgress;

const ERRNO_TIMEDOUT: i64 = -110;
pub fn step_external(
    pid: u32,
    stdin: &mut StdinQueue,
    mut capture: Option<&mut Capture>,
    mut boxed: Option<&mut BoxHold>,
    out: &mut Output<'_>,
    leave_modes: bool,
) -> JobProgress {
    if let Some(progress) = super::box_wait::waiting(boxed.as_deref_mut(), out, leave_modes) {
        return progress;
    }
    stdin.feed(pid);
    let mut buf = [0u8; 256];
    let mut taken = 0;
    while taken < TICK_BUDGET {
        let n = mk_proc_output(pid, buf.as_mut_ptr(), buf.len());
        if n <= 0 {
            break;
        }
        let n = (n as usize).min(buf.len());
        route(&buf[..n], capture.as_deref_mut(), boxed.as_deref_mut(), out);
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
    drain_remaining(pid, capture.as_deref_mut(), boxed.as_deref_mut(), out, &mut buf);
    /* A background job ending while a foreground one holds the screen leaves
     * its modes to that one; otherwise the screen is reset as ever. */
    if !leave_modes {
        out.program_ended();
    }
    let mut status = status as i32;
    /* The program did its part; a file that could not take its output is
     * still a failed command. */
    if let Some(capture) = capture {
        if !capture.finish(out) && status == 0 {
            status = 1;
        }
    }
    super::box_wait::ended(boxed, status, out, leave_modes)
}
