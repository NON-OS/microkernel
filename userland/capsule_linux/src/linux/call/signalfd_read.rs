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

//! A read of a signalfd by one thread: the signals of its mask pending for
//! that thread or its process, EAGAIN on a non-blocking descriptor when there
//! are none, or a wait until one comes.

use super::signalfd_info::SFD_INFO_LEN;
use crate::linux::abi::errno;
use crate::linux::guest::sigwaits::SigWait;
use crate::linux::guest::{Guest, Kind};
use crate::linux::serve::Answer;

use super::signalfd_take::signalfd_take;

/// A read by thread `tid`: what is pending now, EAGAIN, or a parked wait.
pub fn signalfd_read(guest: &mut Guest, tid: u32, fd: u64, buf: u64, len: u64) -> Answer {
    let Some(f) = guest.fds.get(fd as usize).filter(|f| f.kind == Kind::Signal) else {
        return Answer::value(errno::fail(errno::EBADF));
    };
    let (set, nonblock) = (guest.signals.sigfds[f.handle as usize], f.nonblock);
    let records = len / SFD_INFO_LEN as u64;
    if records == 0 {
        return Answer::value(errno::fail(errno::EINVAL));
    }
    if let Some(n) = signalfd_take(guest, tid, set, buf, records) {
        return Answer::value(n);
    }
    if nonblock {
        return Answer::value(errno::fail(errno::EAGAIN));
    }
    guest.signals.sigwaits.push(SigWait { tid, set, info: buf, due: None, records });
    Answer::Park
}
