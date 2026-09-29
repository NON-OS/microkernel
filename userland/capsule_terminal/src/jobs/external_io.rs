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

//! Moving bytes between the terminal and a running program: its stdin
//! queue in, its output out, each bounded a tick.

use alloc::vec::Vec;

use nonos_libc::{mk_proc_input, mk_proc_output};

use crate::command::output::Output;

const MAX_PROC_INPUT: usize = 1024 * 1024;
/// Output taken from a program in one tick. A program printing fast used to
/// be read 256 bytes a tick, about 8 KiB a second; this keeps a frame
/// bounded while letting a build log or a scan scroll at full speed.
pub(super) const TICK_BUDGET: usize = 64 * 1024;

pub(super) fn feed_stdin(pid: u32, in_buf: &mut Vec<u8>, in_cursor: &mut usize) {
    if *in_cursor >= in_buf.len() {
        // All of it went: start the buffer over rather than let it grow.
        in_buf.clear();
        *in_cursor = 0;
        return;
    }
    let pending = &in_buf[*in_cursor..];
    let chunk = &pending[..pending.len().min(MAX_PROC_INPUT)];
    let sent = mk_proc_input(pid as u64, chunk.as_ptr(), chunk.len() as u64);
    if sent > 0 {
        *in_cursor += (sent as usize).min(chunk.len());
    }
}

pub(super) fn drain_remaining(pid: u32, out: &mut Output<'_>, buf: &mut [u8; 256]) {
    let mut taken = 0;
    while taken < TICK_BUDGET {
        let m = mk_proc_output(pid, buf.as_mut_ptr(), buf.len());
        if m <= 0 {
            break;
        }
        let m = (m as usize).min(buf.len());
        out.feed_raw(&buf[..m]);
        taken += m;
    }
}
