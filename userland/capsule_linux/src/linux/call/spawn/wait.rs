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

//! `wait4`: a child that has ended, or a wait until one does. The call only
//! checks what it was asked and parks the caller; the family, which sees
//! every process, answers it at once when a child has ended, with 0 under
//! WNOHANG, or with ECHILD when no child fits, and otherwise when one ends.

use crate::linux::abi::errno;
use crate::linux::guest::sigwaits::{ChildWait, Which};
use crate::linux::guest::Guest;
use crate::linux::serve::Answer;

use super::wait_opts::wait4_options;
pub use super::wait_opts::{WALL, WCLONE, WEXITED, WNOHANG, WNOWAIT};

pub fn wait4(guest: &mut Guest, want: u64, status: u64, flags: u64, tid: u32) -> Answer {
    wait4_usage(guest, want, status, flags, 0, tid)
}

/// wait4 with its rusage: written as zeros, since the kernel reports no CPU
/// time for a guest.
pub fn wait4_usage(
    guest: &mut Guest,
    want: u64,
    status: u64,
    flags: u64,
    rusage: u64,
    tid: u32,
) -> Answer {
    let options = match wait4_options(flags) {
        Ok(options) => options,
        Err(e) => return Answer::value(errno::fail(e)),
    };
    let which = match want as i64 as i32 {
        -1 => Which::Any,
        0 => Which::Group(guest.pgid),
        p if p < 0 => Which::Group(p.unsigned_abs()),
        p => Which::Pid(p as u32),
    };
    park(guest, ChildWait { tid, which, options, out: status, rusage, waitid: false })
}

pub(super) fn park(guest: &mut Guest, w: ChildWait) -> Answer {
    guest.signals.childwaits.push(w);
    Answer::Park
}
