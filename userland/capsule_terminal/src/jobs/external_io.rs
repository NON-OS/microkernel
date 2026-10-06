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

//! Moving a running program's output to the screen, bounded a tick. Its
//! stdin goes the other way through `stdin_queue`.

use nonos_libc::mk_proc_output;

use super::capture::Capture;
use crate::command::output::Output;

/// Output taken from a program in one tick. A program printing fast used to
/// be read 256 bytes a tick, about 8 KiB a second; this keeps a frame
/// bounded while letting a build log or a scan scroll at full speed.
pub(super) const TICK_BUDGET: usize = 64 * 1024;
/// Messages read and dropped from a killed program, at most: four times what
/// an inbox holds by default, and a bound whatever the kernel answers.
const DISCARD_MAX: usize = 4096;

pub(super) fn drain_remaining(
    pid: u32,
    mut capture: Option<&mut Capture>,
    out: &mut Output<'_>,
    buf: &mut [u8; 256],
) {
    /* Output bound for a file is all taken now, up to four times what an
     * inbox holds: what is left after the job ends is never read. The
     * screen keeps its per-tick bound. */
    let budget = if capture.is_some() { DISCARD_MAX * buf.len() } else { TICK_BUDGET };
    let mut taken = 0;
    while taken < budget {
        let m = mk_proc_output(pid, buf.as_mut_ptr(), buf.len());
        if m <= 0 {
            break;
        }
        let m = (m as usize).min(buf.len());
        match capture.as_deref_mut() {
            Some(capture) => capture.route(&buf[..m], out),
            None => out.feed_raw(&buf[..m]),
        }
        taken += m;
    }
}

/// Read what a killed program had written and not yet shown, and drop it.
/// The kernel keeps a dead program's output until its parent reads it dry;
/// a chat's words should not wait there for a reader that is not coming.
pub(super) fn discard_output(pid: u32) {
    let mut buf = [0u8; 256];
    for _ in 0..DISCARD_MAX {
        if mk_proc_output(pid, buf.as_mut_ptr(), buf.len()) <= 0 {
            break;
        }
    }
}
