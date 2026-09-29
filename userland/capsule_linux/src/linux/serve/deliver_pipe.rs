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

//! SIGPIPE at the thread whose write found no reader. Linux raises it in the
//! write itself; here the write answers EPIPE and the answer is where the
//! signal is raised, before the reply, so the thread takes it on its way out
//! as Linux's does. A send with MSG_NOSIGNAL raises nothing.

use crate::linux::abi::errno;
use crate::linux::call::sigpipe;
use crate::linux::guest::Guest;

const WRITE: u64 = 1;
const WRITEV: u64 = 20;
const SENDTO: u64 = 44;
const SENDMSG: u64 = 46;
const MSG_NOSIGNAL: u64 = 0x4000;

/// Raise SIGPIPE at `tid` when call `nr` with arguments `a` answered EPIPE.
pub fn broken_pipe(g: &mut Guest, tid: u32, nr: u64, a: [u64; 6], value: u64) {
    if value != errno::fail(errno::EPIPE) {
        return;
    }
    let quiet = match nr {
        WRITE | WRITEV => false,
        SENDTO => a[3] & MSG_NOSIGNAL != 0,
        SENDMSG => a[2] & MSG_NOSIGNAL != 0,
        _ => return,
    };
    if !quiet {
        sigpipe(g, tid);
    }
}
