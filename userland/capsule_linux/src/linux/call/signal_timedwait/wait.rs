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

//! rt_sigtimedwait: a pending signal of the set taken now, else the thread
//! parked until one comes or its time runs out.

use super::super::signal_act::wait_set;
use super::super::signal_wait::SIGSET_LEN;
use super::timeout::wait_ms;
use crate::linux::abi::errno;
use crate::linux::call::now_ms;
use crate::linux::guest::sigwaits::SigWait;
use crate::linux::guest::Guest;
use crate::linux::serve::Answer;

const CLOCK_MONOTONIC: u64 = 1;

pub fn rt_sigtimedwait(
    guest: &mut Guest,
    tid: u32,
    set: u64,
    info: u64,
    ts: u64,
    size: u64,
) -> Answer {
    if size != SIGSET_LEN {
        return Answer::value(errno::fail(errno::EINVAL));
    }
    let Some(want) = wait_set(guest, set) else {
        return Answer::value(errno::fail(errno::EFAULT));
    };
    let due = match ts {
        0 => None,
        at => match wait_ms(guest, at) {
            Ok(ms) => now_ms(CLOCK_MONOTONIC).map(|now| now.saturating_add(ms)),
            Err(e) => return Answer::value(e),
        },
    };
    /* A signal of the set already waiting is taken now, whatever the timeout. */
    if let Some(got) = guest.signals.take(tid, want) {
        let bytes = got.bytes();
        if info != 0 && guest.write(info, &bytes) < bytes.len() as i64 {
            return Answer::value(errno::fail(errno::EFAULT));
        }
        return Answer::value(errno::ok(u64::from(got.signo)));
    }
    if due.is_some_and(|d| now_ms(CLOCK_MONOTONIC).is_some_and(|now| d <= now)) {
        return Answer::value(errno::fail(errno::EAGAIN));
    }
    guest.signals.sigwaits.push(SigWait { tid, set: want, info, due, records: 0 });
    Answer::Park
}
