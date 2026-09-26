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

//! Raw Linux syscalls. The probes pass exact bytes, a path with a NUL in it
//! included, which std refuses to send, so nothing here goes through libc.

use core::arch::asm;

pub const READ: u64 = 0;
pub const WRITE: u64 = 1;
pub const OPEN: u64 = 2;
pub const CLOSE: u64 = 3;
pub const MMAP: u64 = 9;
pub const MPROTECT: u64 = 10;
pub const BRK: u64 = 12;
pub const NANOSLEEP: u64 = 35;
pub const GETPID: u64 = 39;
pub const KILL: u64 = 62;
pub const PTRACE: u64 = 101;
pub const SYMLINK: u64 = 88;
pub const OPENAT: u64 = 257;
pub const PROCESS_VM_READV: u64 = 310;

pub const AT_FDCWD: i64 = -100;

pub const PROT_RW: u64 = 0x3;
pub const MAP_PRIVATE_ANON: u64 = 0x22;
pub const MAP_FIXED: u64 = 0x10;

/// Where the memory pair meet. Any address works; both must agree on it.
pub const PATTERN_AT: u64 = 0x5000_0000;
pub const PATTERN: &[u8; 16] = b"NONOS-SIBLING-01";

/// One syscall with up to six arguments. The return is the raw value: a
/// negative errno on failure, as the kernel ABI gives it.
pub fn call(nr: u64, a: [u64; 6]) -> i64 {
    let ret: i64;
    // SAFETY: the syscall instruction clobbers rcx and r11 and reads its
    // arguments from the registers named here. Pointers passed in `a` are
    // the caller's; a bad one is what a probe is for, and the other side
    // either refuses it or faults this process, never the machine.
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") nr as i64 => ret,
            in("rdi") a[0], in("rsi") a[1], in("rdx") a[2],
            in("r10") a[3], in("r8") a[4], in("r9") a[5],
            lateout("rcx") _, lateout("r11") _,
            options(nostack),
        );
    }
    ret
}

/// Write bytes to stdout. The guests have no allocator beyond std's and
/// report through the one channel every personality gives a program.
pub fn out(bytes: &[u8]) {
    let _ = call(WRITE, [1, bytes.as_ptr() as u64, bytes.len() as u64, 0, 0, 0]);
}
