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

//! `waitid`: wait4's question asked by id type, answered in a siginfo, with
//! WNOWAIT to look without reaping. Linux writes the siginfo whatever the
//! outcome, zeros when nothing is reported, so an error here writes it too.

use crate::linux::abi::errno;
use crate::linux::guest::sigwaits::{ChildWait, Which};
use crate::linux::guest::Guest;
use crate::linux::serve::Answer;

use super::wait::park;
use super::wait_opts::waitid_options;

const P_ALL: u64 = 0;
const P_PID: u64 = 1;
const P_PGID: u64 = 2;
const P_PIDFD: u64 = 3;
pub const SIGINFO_LEN: usize = 128;

pub fn waitid(guest: &mut Guest, tid: u32, a: [u64; 6]) -> Answer {
    let (idtype, id, info, options, rusage) = (a[0], a[1] as u32, a[2], a[3], a[4]);
    let options = match waitid_options(options) {
        Ok(options) => options,
        Err(e) => return refuse(guest, info, e),
    };
    let which = match idtype {
        P_ALL => Which::Any,
        P_PID if (id as i32) > 0 => Which::Pid(id),
        P_PGID if id == 0 => Which::Group(guest.pgid),
        P_PGID if (id as i32) > 0 => Which::Group(id),
        /* No descriptor here is ever a pidfd: pidfd_open is not served. */
        P_PIDFD if (id as i32) >= 0 => return refuse(guest, info, errno::EBADF),
        _ => return refuse(guest, info, errno::EINVAL),
    };
    park(guest, ChildWait { tid, which, options, out: info, rusage, waitid: true })
}

fn refuse(guest: &Guest, info: u64, e: i64) -> Answer {
    if info != 0 && guest.write(info, &[0u8; SIGINFO_LEN]) < SIGINFO_LEN as i64 {
        return Answer::value(errno::fail(errno::EFAULT));
    }
    Answer::value(errno::fail(e))
}
