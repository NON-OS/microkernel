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

//! One refused call, answered. Every number a guest can ask for arrives
//! here; the ones with no handler yet are named on the log and refused
//! with `ENOSYS`, so what is missing is a list and never a guess.

use nonos_libc::ForeignFrame;

use crate::linux::abi::{errno, name, nr};
use crate::linux::call;
use crate::linux::guest::Guest;

pub fn answer(guest: &mut Guest, frame: &ForeignFrame) -> u64 {
    let a = frame.args();
    match frame.nr {
        nr::WRITE => call::write(guest, a[0], a[1], a[2]),
        nr::WRITEV => call::writev(guest, a[0], a[1], a[2]),
        nr::READ => call::read(guest, a[0], a[1], a[2]),
        nr::CLOSE => call::close(guest, a[0]),
        nr::BRK => call::brk(guest, a[0]),
        nr::MMAP => call::mmap(guest, a[0], a[1], a[2], a[3]),
        nr::MUNMAP => call::munmap(guest, a[0], a[1]),
        nr::MPROTECT | nr::MADVISE | nr::RSEQ | nr::SET_ROBUST_LIST => errno::ok(0),
        nr::ARCH_PRCTL => call::arch_prctl(guest, a[0], a[1]),
        nr::SET_TID_ADDRESS | nr::GETTID | nr::GETPID => errno::ok(guest.pid as u64),
        nr::GETUID | nr::GETEUID | nr::GETGID | nr::GETEGID => errno::ok(0),
        nr::CLOCK_GETTIME => call::clock_gettime(guest, a[0], a[1]),
        nr::GETRANDOM => call::getrandom(guest, a[0], a[1], a[2]),
        nr::EXIT | nr::EXIT_GROUP => call::exit(guest, a[0]),
        other => unserved(other),
    }
}

/// Name what was asked for. A guest that dies on a missing call should
/// leave behind the name of the call it needed.
fn unserved(number: u64) -> u64 {
    let mut line = [0u8; 64];
    let head = b"[LINUX] unserved ";
    let tag = name::of(number);
    let n = head.len().min(line.len());
    line[..n].copy_from_slice(&head[..n]);
    let m = (n + tag.len()).min(line.len());
    line[n..m].copy_from_slice(&tag[..m - n]);
    let end = (m + 1).min(line.len());
    line[m..end].copy_from_slice(b"\n");
    let _ = nonos_libc::mk_debug(line.as_ptr(), end);
    errno::fail(errno::ENOSYS)
}
