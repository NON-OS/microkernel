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

//! Taking signals for a signalfd read: as many pending signals of the mask as
//! records fit, each written where the next record goes. None when nothing is
//! pending, so the read can wait or answer EAGAIN.

use super::signalfd_info::SFD_INFO_LEN;
use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

pub fn signalfd_take(guest: &mut Guest, tid: u32, set: u64, buf: u64, records: u64) -> Option<u64> {
    let mut n = 0;
    while n < records {
        let Some(info) = guest.signals.take(tid, set) else {
            break;
        };
        let at = buf + n * SFD_INFO_LEN as u64;
        if guest.write(at, &info.fd_bytes()) < SFD_INFO_LEN as i64 {
            return Some(if n == 0 { errno::fail(errno::EFAULT) } else { n * SFD_INFO_LEN as u64 });
        }
        n += 1;
    }
    (n > 0).then_some(n * SFD_INFO_LEN as u64)
}

/// poll's POLLIN when a signal of the mask waits for the process or its
/// first thread; a read by another thread may find its own too.
pub fn signalfd_bits(guest: &Guest, fd: u64) -> u16 {
    const POLLIN: u16 = 0x001;
    let Some(f) = guest.fds.get(fd as usize).filter(|f| f.kind == Kind::Signal) else {
        return 0;
    };
    let set = guest.signals.sigfds.get(f.handle as usize).copied().unwrap_or(0);
    let who = guest.live_threads().first().copied().unwrap_or(guest.pid);
    if guest.signals.pending_for(who) & set != 0 {
        POLLIN
    } else {
        0
    }
}

/// A read that reaches a signalfd another way than read(2), readv: what is
/// pending for the process or its first thread now, else EAGAIN.
pub fn signalfd_now(guest: &mut Guest, fd: u64, buf: u64, len: u64) -> u64 {
    let Some(f) = guest.fds.get(fd as usize).filter(|f| f.kind == Kind::Signal) else {
        return errno::fail(errno::EBADF);
    };
    let set = guest.signals.sigfds.get(f.handle as usize).copied().unwrap_or(0);
    let who = guest.live_threads().first().copied().unwrap_or(guest.pid);
    let records = len / SFD_INFO_LEN as u64;
    match records {
        0 => errno::fail(errno::EINVAL),
        n => signalfd_take(guest, who, set, buf, n).unwrap_or(errno::fail(errno::EAGAIN)),
    }
}
