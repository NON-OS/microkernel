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

//! `signalfd4` and `signalfd`: a descriptor that reads the signals of a mask
//! pending for the reading thread or its process, as `signalfd_siginfo`
//! records, instead of their handlers running. A read with none pending
//! answers EAGAIN on a non-blocking descriptor and otherwise parks the
//! thread until one comes, as Linux's does.

use crate::linux::abi::errno;
use crate::linux::file::install;
use crate::linux::guest::sigstate::blockable;
use crate::linux::guest::{Fd, Guest, Kind};

const SFD_NONBLOCK: u64 = 0o4000;
const SFD_CLOEXEC: u64 = 0o2000000;
const SIGSET_LEN: u64 = 8;

pub fn signalfd4(guest: &mut Guest, fd: u64, mask: u64, size: u64, flags: u64) -> u64 {
    if size != SIGSET_LEN || flags & !(SFD_NONBLOCK | SFD_CLOEXEC) != 0 {
        return errno::fail(errno::EINVAL);
    }
    let Some(raw) = guest.read(mask, 8) else {
        return errno::fail(errno::EFAULT);
    };
    let set = blockable(u64::from_le_bytes(raw[..8].try_into().unwrap_or([0; 8])));
    if fd as i64 != -1 {
        /* An existing signalfd takes the new mask; any other descriptor is refused. */
        return match guest.fds.get(fd as usize).filter(|f| f.kind == Kind::Signal) {
            Some(f) => {
                let at = f.handle as usize;
                guest.signals.sigfds[at] = set;
                errno::ok(fd)
            }
            None => errno::fail(errno::EINVAL),
        };
    }
    guest.signals.sigfds.push(set);
    let mut new = Fd::empty(Kind::Signal);
    new.handle = (guest.signals.sigfds.len() - 1) as u32;
    new.nonblock = flags & SFD_NONBLOCK != 0;
    new.cloexec = flags & SFD_CLOEXEC != 0;
    match install(guest, new) {
        Some(n) => errno::ok(n),
        None => errno::fail(errno::EMFILE),
    }
}
