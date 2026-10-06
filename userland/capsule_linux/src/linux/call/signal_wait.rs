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

//! Waiting for a signal. pause and rt_sigsuspend park until a handler runs,
//! and answer EINTR through it; sigsuspend waits under the mask it was given
//! and the handler's return puts the old one back. rt_sigpending names what
//! waits; rt_sigtimedwait is in signal_timedwait.

use crate::linux::abi::errno;
use crate::linux::guest::sigwaits::SigWait;
use crate::linux::guest::Guest;
use crate::linux::serve::Answer;

pub const SIGSET_LEN: u64 = 8;

pub fn pause(guest: &mut Guest, tid: u32) -> Answer {
    guest.signals.sigwaits.push(SigWait { tid, set: 0, info: 0, due: None, records: 0 });
    Answer::Park
}

pub fn rt_sigsuspend(guest: &mut Guest, tid: u32, set: u64, size: u64) -> Answer {
    if size != SIGSET_LEN {
        return Answer::value(errno::fail(errno::EINVAL));
    }
    let Some(mask) = read_set(guest, set) else {
        return Answer::value(errno::fail(errno::EFAULT));
    };
    let old = guest.signals.blocked(tid);
    guest.signals.set_blocked(tid, mask);
    guest.signals.thread(tid).saved = Some(old);
    pause(guest, tid)
}

pub fn rt_sigpending(guest: &mut Guest, tid: u32, out: u64, size: u64) -> u64 {
    if size > SIGSET_LEN {
        return errno::fail(errno::EINVAL);
    }
    let held = guest.signals.pending_for(tid) & guest.signals.blocked(tid);
    match guest.write(out, &held.to_le_bytes()[..size as usize]) < size as i64 {
        true => errno::fail(errno::EFAULT),
        false => errno::ok(0),
    }
}

pub fn read_set(guest: &Guest, at: u64) -> Option<u64> {
    guest.read(at, 8).map(|raw| u64::from_le_bytes(raw[..8].try_into().unwrap_or([0; 8])))
}
