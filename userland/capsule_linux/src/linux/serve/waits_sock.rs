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

//! Socket calls that wait: accept, connect, the sends and receives, and a
//! read or write on a socket. Each is tried at once; a blocking one that
//! cannot finish is parked, and the family tries it again after every
//! answer (`family_waits`). A family socket changes only in an answer, so
//! it needs no tick; a stream net.sockets holds is looked at on one.
//!
//! SO_RCVTIMEO and SO_SNDTIMEO bound the wait, which then answers EAGAIN,
//! or the count moved so far, as Linux does.

use crate::linux::abi::{errno, nr};
use crate::linux::call::now_ms;
use crate::linux::guest::{Blocked, Guest};
use crate::linux::net::{self, sock};

use super::answer::Answer;
pub use super::waits_sock_kind::{takes, ticks};

const CLOCK_MONOTONIC: u64 = 1;
const MSG_DONTWAIT: u64 = 0x40;

pub fn io(guest: &mut Guest, tid: u32, n: u64, a: [u64; 6]) -> Answer {
    let reads = !matches!(
        n,
        nr::WRITE | nr::WRITEV | nr::SENDTO | nr::SENDMSG | nr::SENDMMSG | nr::CONNECT
    );
    let limit = net::sock_id(guest, a[0]).and_then(|id| net::limit_ms(id, reads));
    let deadline = limit.map(|ms| now_ms(CLOCK_MONOTONIC).unwrap_or(0).saturating_add(ms));
    let wait = Blocked { tid, nr: n, args: a, deadline };
    match attempt(guest, &wait) {
        Some(v) => Answer::value(v),
        None => {
            guest.blocked.push(wait);
            Answer::Park
        }
    }
}

/// The call's answer if it can give one now, None to go on waiting.
pub fn attempt(guest: &mut Guest, wait: &Blocked) -> Option<u64> {
    let (n, a, tid) = (wait.nr, wait.args, wait.tid);
    let done = sock::progress(tid);
    let (value, whole) = net::try_call(guest, n, a, done);
    let flags = net::call_flags(n, a);
    let blocking =
        flags & MSG_DONTWAIT == 0 && !guest.fds.get(a[0] as usize).is_some_and(|f| f.nonblock);
    let late = wait.deadline.is_some_and(|d| now_ms(CLOCK_MONOTONIC).is_some_and(|now| d <= now));
    let answer = match errno::slot(value) {
        Some(moved) => {
            let total = done + moved;
            let stream = net::sock_id(guest, a[0]).is_some_and(net::is_stream);
            if blocking && moved != 0 && total < whole && !late && net::wants_all(stream, n, flags)
            {
                sock::set_progress(tid, total);
                return None;
            }
            total as u64
        }
        None if value == errno::fail(errno::EAGAIN) && blocking && !late => return None,
        // What moved before an error or the time limit is what Linux answers.
        None if done != 0 => done as u64,
        None => value,
    };
    sock::set_progress(tid, 0);
    Some(answer)
}
