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

//! A read on an empty pipe waits, as Linux's does.
//!
//! Whether anything could still fill the pipe is the family's to say, since
//! the write end may be in another process: the read parks here and the serve
//! loop settles it, with bytes when some arrive or end of file when no write
//! end is left anywhere.

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};
use crate::linux::serve::Answer;

use super::pipe_end::{end_of, other_end_open};
use super::pipe_read::read;

pub fn is_pipe(guest: &Guest, fd: u64) -> bool {
    guest.fds.get(fd as usize).is_some_and(|f| f.kind == Kind::Pipe)
}

pub fn read_or_park(guest: &mut Guest, fd: u64, buf: u64, len: u64, tid: u32) -> Answer {
    let Some((slot, writable)) = end_of(guest, fd) else {
        return Answer::value(errno::fail(errno::EBADF));
    };
    let fed = !guest.pipes[slot].is_empty() || !other_end_open(guest, slot, false);
    if writable || len == 0 || fed {
        return Answer::value(read(guest, fd, buf, len));
    }
    guest.pipe_wait = Some((slot, buf, len, tid));
    Answer::Park
}
