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

//! `kill`, `tkill` and `tgkill`. A signal for the caller's own
//! process is queued here and taken as `serve::deliver` decides; one for
//! another process of the family leaves through the outbox with the caller
//! parked, and the family answers it once it knows whether anyone was there.
//! A guest reaches only its own family: pid_map refuses any other number.

use super::signal_post::post;
use crate::linux::abi::errno;
use crate::linux::guest::siginfo::{SigInfo, SI_TKILL, SI_USER};
use crate::linux::guest::sigstate::NSIG;
use crate::linux::guest::sigwaits::Target;
use crate::linux::guest::Guest;
use crate::linux::serve::Answer;

/// kill(pid, sig) from thread `tid`, which parks when the family must answer.
pub fn kill_from(guest: &mut Guest, tid: u32, pid: u64, signo: u64) -> Answer {
    let to = match pid as i64 {
        -1 => Target::All,
        0 => Target::Group(guest.pgid),
        p if p < 0 => Target::Group(p.unsigned_abs() as u32),
        p => Target::Process(p as u32),
    };
    send(guest, tid, to, signo, SI_USER)
}

/// kill for a caller that cannot park: a signal for another process still
/// goes to the family, with no one to answer, so a target that is gone reads
/// as 0 here rather than ESRCH.
pub fn kill(guest: &mut Guest, pid: u64, signo: u64) -> u64 {
    match kill_from(guest, 0, pid, signo) {
        Answer::Reply(v) => v,
        Answer::Park => errno::ok(0),
    }
}

/// tkill(tid, sig), and tgkill(tgid, tid, sig) with `tgid` non-zero.
pub fn tgkill_from(guest: &mut Guest, tid: u32, tgid: u64, target: u64, signo: u64) -> Answer {
    if (tgid as i64) < 0 || (target as i64) <= 0 {
        return Answer::value(errno::fail(errno::EINVAL));
    }
    send(guest, tid, Target::Thread(tgid as u32, target as u32), signo, SI_TKILL)
}

fn send(guest: &mut Guest, tid: u32, to: Target, signo: u64, code: i32) -> Answer {
    if signo > NSIG as u64 {
        return Answer::value(errno::fail(errno::EINVAL));
    }
    let info = SigInfo::from(signo as u8, code, guest.pid);
    post(guest, tid, to, info)
}
