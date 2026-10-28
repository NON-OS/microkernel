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

//! `get_robust_list`. set_robust_list is taken and nothing is kept
//! (serve/table/all.rs): no thread's death walks a list here, and a mutex a
//! dead owner held is set up again by the program that next takes its file
//! alone, as LMDB does its lock file. So a thread's own list reads as none,
//! 24 bytes long, the size of struct robust_list_head. musl's
//! pthread_mutexattr_setrobust asks exactly this before it makes a mutex
//! robust; unserved, LMDB could not set up its lock file, and the Monero
//! wallet said "Failed to initialize ring database". Another thread's list
//! is refused: whose list it is says nothing a guest should learn.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

/// The size of struct robust_list_head on x86-64: two pointers and a long.
const HEAD_BYTES: u64 = 24;

pub fn get_robust_list(guest: &Guest, pid: u64, head: u64, len: u64) -> u64 {
    if pid != 0 {
        return errno::fail(errno::EPERM);
    }
    let wrote = guest.write(head, &0u64.to_le_bytes()) == 8
        && guest.write(len, &HEAD_BYTES.to_le_bytes()) == 8;
    match wrote {
        true => errno::ok(0),
        false => errno::fail(errno::EFAULT),
    }
}
