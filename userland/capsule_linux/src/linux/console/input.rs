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

//! A guest reading the terminal: what the person typed, taken from the
//! personality's stdin inbox into the family's queue as it is asked for.

use nonos_libc::mk_stdin_read;

use super::queue_piece::Taken;
use super::state::{attached, with};
use super::wipe::wipe;
use crate::linux::abi::errno;
use crate::linux::guest::Guest;

/// One inbox message at most; the terminal sends a line at a time.
const SCRATCH: usize = 4096;

/// Move what the terminal has sent into the queue, until its inbox is
/// empty or the queue is full. The scratch copy is zeroed after each.
pub(super) fn pull() {
    let mut scratch = [0u8; SCRATCH];
    while !with(|c| c.queue.full()) {
        let n = mk_stdin_read(scratch.as_mut_ptr(), SCRATCH as u64);
        let Some(n) = usize::try_from(n).ok().filter(|&n| n > 0) else {
            break;
        };
        let got = &mut scratch[..n.min(SCRATCH)];
        with(|c| c.queue.push(got));
        wipe(got);
    }
}

/// `read` of the console's input: what was typed, up to a line; an end of
/// file once for each typed; EAGAIN, which parks the reader, when nothing
/// is waiting. A family on no terminal is typed nothing: end of file.
pub fn read(guest: &Guest, buf: u64, len: u64) -> u64 {
    if !attached() || len == 0 {
        return errno::ok(0);
    }
    pull();
    match with(|c| c.queue.take(len.min(SCRATCH as u64) as usize)) {
        Taken::Empty => errno::fail(errno::EAGAIN),
        Taken::Eof => errno::ok(0),
        Taken::Bytes(mut bytes) => {
            let wrote = guest.write(buf, &bytes);
            let n = bytes.len() as u64;
            wipe(&mut bytes);
            match wrote == n as i64 {
                true => errno::ok(n),
                false => errno::fail(errno::EFAULT),
            }
        }
    }
}
