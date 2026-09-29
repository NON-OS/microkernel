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

use nonos_libc::{mk_proc_output, mk_wait};

use crate::command::output::Output;
use super::external_io::{drain_remaining, feed_stdin, TICK_BUDGET};
use crate::jobs::JobProgress;

const ERRNO_TIMEDOUT: i64 = -110;
pub fn step_external(
    pid: u32,
    in_buf: &mut Vec<u8>,
    in_cursor: &mut usize,
    out: &mut Output<'_>,
    leave_modes: bool,
) -> JobProgress {
    feed_stdin(pid, in_buf, in_cursor);
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
    in_buf.extend_from_slice(&out.take_replies());
    let status = mk_wait(pid as u64, 0);
    if status == ERRNO_TIMEDOUT {
        return JobProgress::Running;
    }
    drain_remaining(pid, out, &mut buf);
    /* A background job ending while a foreground one holds the screen leaves
     * its modes to that one; otherwise the screen is reset as ever. */
    if !leave_modes {
        out.program_ended();
    }
    JobProgress::Done(status as i32)
}
