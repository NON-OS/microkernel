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

//! `rt_sigqueueinfo` and `rt_tgsigqueueinfo`: a signal with the siginfo the
//! caller wrote, which is how sigqueue hands a value along. As on Linux, only
//! a process signalling itself may claim a kernel code (si_code 0 or above) or
//! SI_TKILL; the signal number is the call's, whatever the siginfo says.

use crate::linux::abi::errno;
use crate::linux::guest::siginfo::{SigInfo, SI_TKILL};
use crate::linux::guest::sigstate::NSIG;
use crate::linux::guest::sigwaits::Target;
use crate::linux::guest::Guest;
use crate::linux::serve::Answer;

pub fn rt_sigqueueinfo(guest: &mut Guest, tid: u32, tgid: u64, sig: u64, uinfo: u64) -> Answer {
    queue(guest, tid, Target::Process(tgid as u32), tgid, sig, uinfo)
}

pub fn rt_tgsigqueueinfo(
    guest: &mut Guest,
    tid: u32,
    tgid: u64,
    target: u64,
    sig: u64,
    uinfo: u64,
) -> Answer {
    if (tgid as i64) <= 0 || (target as i64) <= 0 {
        return Answer::value(errno::fail(errno::EINVAL));
    }
    queue(guest, tid, Target::Thread(tgid as u32, target as u32), tgid, sig, uinfo)
}

fn queue(guest: &mut Guest, tid: u32, to: Target, tgid: u64, sig: u64, uinfo: u64) -> Answer {
    if sig > NSIG as u64 {
        return Answer::value(errno::fail(errno::EINVAL));
    }
    let Some(raw) = guest.read(uinfo, 32) else {
        return Answer::value(errno::fail(errno::EFAULT));
    };
    let word = |i: usize| u32::from_le_bytes(raw[i..i + 4].try_into().unwrap_or([0; 4]));
    let code = word(8) as i32;
    /* Linux compares the caller's own thread with the one it names. */
    let named = match to {
        Target::Thread(_, t) => t,
        _ => tgid as u32,
    };
    if (code >= 0 || code == SI_TKILL) && named != tid {
        return Answer::value(errno::fail(errno::EPERM));
    }
    let info = SigInfo {
        signo: sig as u8,
        code,
        pid: crate::linux::serve::kernel_pid(word(16)).unwrap_or(0),
        timer: None,
        value: u64::from_le_bytes(raw[24..32].try_into().unwrap_or([0; 8])),
    };
    super::signal_post::post(guest, tid, to, info)
}
