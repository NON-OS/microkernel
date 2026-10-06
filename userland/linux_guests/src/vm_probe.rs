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

//! Reading or steering a sibling through the calls built for it.

use crate::report::{Report, Seen};
use crate::sys::{call, KILL, PATTERN_AT, PROCESS_VM_READV, PTRACE};

/// Attaches without stopping the target, so a success does not wedge it.
const PTRACE_SEIZE: u64 = 0x4206;

/// process_vm_readv, ptrace and kill against every pid. Each reports once:
/// the first escape it finds, or the errno every pid gave.
pub fn scan(r: &mut Report, pids: &[u32]) {
    r.check("process_vm_readv", sweep(pids, read_sibling));
    r.check(
        "ptrace seize",
        sweep(pids, |pid| call(PTRACE, [PTRACE_SEIZE, pid as u64, 0, 0, 0, 0])),
    );
    // Signal 0 delivers nothing and only answers whether the pid exists, so
    // a yes is a disclosure; SIGKILL would end the holder and the run.
    r.check("kill probe", sweep(pids, |pid| call(KILL, [pid as u64, 0, 0, 0, 0, 0])));
}

fn read_sibling(pid: u32) -> i64 {
    let mut buf = [0u8; 16];
    let local = [buf.as_mut_ptr() as u64, 16u64];
    let remote = [PATTERN_AT, 16u64];
    // Any read that succeeds is an escape, whatever it brings back.
    call(PROCESS_VM_READV, [pid as u64, local.as_ptr() as u64, 1, remote.as_ptr() as u64, 1, 0])
}

/// The first pid a call succeeded against, or the last errno if none did.
fn sweep(pids: &[u32], f: impl Fn(u32) -> i64) -> Seen {
    let mut last = -1;
    for &pid in pids {
        let rc = f(pid);
        if rc >= 0 {
            return Seen::Escaped(format!("pid {pid} answered {rc}"));
        }
        last = rc;
    }
    Seen::Refused(last)
}
