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

//! What an interrupted call answers, as Linux decides it. Reads and writes,
//! the socket calls, wait4, waitid and an untimed futex wait end with
//! ERESTARTSYS: under SA_RESTART the call runs again once the handler returns,
//! otherwise EINTR. Sleeps, timed waits, poll, select, epoll, pause and the
//! signal waits always answer EINTR, and a relative nanosleep writes the time
//! it had left (deliver_rem).

use nonos_libc::ForeignRegs;

use crate::linux::abi::errno;
use crate::linux::guest::sigstate::SA_RESTART;

pub const R10: usize = 2;
pub const RSI: usize = 9;
pub const RAX: usize = 13;
const RIP: usize = 16;
/// The length of `syscall`, which a restart steps back over.
const SYSCALL_LEN: u64 = 2;

/// read, write, readv, writev, accept, sendto, recvfrom, sendmsg, recvmsg,
/// wait4, waitid, accept4.
const RESTARTS: [u64; 12] = [0, 1, 19, 20, 43, 44, 45, 46, 47, 61, 247, 288];
const FUTEX: u64 = 202;

/// Set rax to what the handler returns into: the call again, or EINTR. The
/// parked frame's rax is still the call's number.
pub fn rewind(regs: &mut ForeignRegs, flags: u64) {
    let nr = regs[RAX];
    let untimed_futex = nr == FUTEX && regs[R10] == 0;
    if flags & SA_RESTART != 0 && (RESTARTS.contains(&nr) || untimed_futex) {
        regs[RIP] = regs[RIP].wrapping_sub(SYSCALL_LEN);
        return;
    }
    regs[RAX] = errno::fail(errno::EINTR);
}
