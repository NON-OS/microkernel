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

//! `epoll_wait`: which of the watched descriptors are ready now.
//!
//! A level-triggered entry is reported for as long as its readiness holds.
//! An EPOLLET entry is reported when readiness rises: bits already seen at
//! the last look are left out until they fall, or until a call on the
//! descriptor answers EAGAIN (`epoll_arm`). An EPOLLONESHOT entry reports
//! once and then nothing until it is modified.

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind, EPOLLET, EPOLLONESHOT};
use crate::linux::net::{ready, POLLERR, POLLHUP};

use super::epoll::EVENT_LEN;

/// The most events one call can ask for, as Linux bounds it.
const MOST: u64 = (i32::MAX as u64) / EVENT_LEN as u64;

/// Report what is ready now, never waiting; `waits` does the waiting.
pub fn epoll_wait(guest: &mut Guest, ep: u64, out: u64, max: u64) -> u64 {
    // maxevents is an int, and one of zero or less is refused.
    if max == 0 || max > MOST {
        return errno::fail(errno::EINVAL);
    }
    let Some(list) = guest.fds.get(ep as usize).filter(|f| f.kind == Kind::Epoll) else {
        return errno::fail(errno::EBADF);
    };
    let mut watch = list.watch.clone();
    let mut blob: Vec<u8> = Vec::new();
    let mut hits = 0u64;
    for w in watch.iter_mut().filter(|w| w.armed) {
        if hits >= max {
            break;
        }
        // Hang-up and error are reported whether they were asked for or not.
        let level = u32::from(ready(guest, w.fd)) & (w.events | u32::from(POLLHUP | POLLERR));
        let live = if w.events & EPOLLET != 0 { level & !w.fired } else { level };
        w.fired = level;
        if live == 0 {
            continue;
        }
        w.armed = w.events & EPOLLONESHOT == 0;
        blob.extend_from_slice(&live.to_le_bytes());
        blob.extend_from_slice(&w.data.to_le_bytes());
        hits += 1;
    }
    if !blob.is_empty() && guest.write(out, &blob) < blob.len() as i64 {
        return errno::fail(errno::EFAULT);
    }
    // What was reported, and what was seen, is kept only once it is delivered.
    if let Some(list) = guest.fds.get_mut(ep as usize) {
        list.watch = watch;
    }
    errno::ok(hits)
}
