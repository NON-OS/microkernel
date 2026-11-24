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

//! A personality's argv as the kernel set it at spawn, NUL-separated: the
//! words its Terminal typed. Only the spawn path writes a process's argv
//! (capsule_spawn's admit and the terminal slots' admit, both before its
//! first instruction); no system call does, so neither the personality nor
//! any guest can change what this reads.

use alloc::vec::Vec;

use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_SRCH};
use crate::usercopy::copy_to_user;

/// Copy `pid`'s argv to `ptr`, at most `len` bytes; its whole length.
pub(super) fn copy_out(pid: u32, ptr: u64, len: u64) -> i64 {
    let Some(words) = crate::process::with_process(pid, |pcb| pcb.argv.lock().clone()) else {
        return ERRNO_SRCH;
    };
    let mut blob = Vec::new();
    for (i, word) in words.iter().enumerate() {
        if i > 0 {
            blob.push(0);
        }
        blob.extend_from_slice(word.as_bytes());
    }
    let n = blob.len().min(usize::try_from(len).unwrap_or(usize::MAX));
    if n > 0 && copy_to_user(ptr, &blob[..n]).is_err() {
        return ERRNO_FAULT;
    }
    blob.len() as i64
}
