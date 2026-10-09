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

use alloc::vec::Vec;

use super::queue::{tgid_of, FUTEX_QUEUE};
use crate::process::current_pid;
use crate::syscall::microkernel::errnos::ERRNO_PERM;

/// Wake up to `count` waiters on `vaddr` (0 means all). Returns the number
/// woken.
pub fn sys_futex_wake(vaddr: u64, count: u64) -> i64 {
    let caller = current_pid().unwrap_or(0);
    if caller == 0 {
        return ERRNO_PERM;
    }
    let key = (tgid_of(caller), vaddr);
    // Collect the waiters, then release the queue lock before touching the
    // scheduler, so the futex lock never nests inside the run-queue lock.
    let to_wake: Vec<u32> = {
        let mut q = FUTEX_QUEUE.lock();
        match q.get_mut(&key) {
            Some(v) => {
                let n = if count == 0 { v.len() } else { (count as usize).min(v.len()) };
                let drained: Vec<u32> = v.drain(..n).collect();
                if v.is_empty() {
                    q.remove(&key);
                }
                drained
            }
            None => Vec::new(),
        }
    };
    let woke = to_wake.len() as i64;
    for pid in to_wake {
        crate::sched::wake_process(pid);
    }
    woke
}
