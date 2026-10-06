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

use super::queue::{tgid_of, FUTEX_QUEUE, MAX_TIMED_MS, SAFETY_MS};
use super::waiters::{join, leave, leave_early};
use crate::process::current_pid;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_PERM};

/// Block until the 32-bit word at `vaddr` no longer holds `expected`, a
/// waker signals it, or (with a nonzero `timeout_ms`) the timeout lapses.
/// Returns 0 in every non-error case; the caller rechecks the atomic and
/// re-waits if needed, which is why a spurious early return is harmless.
pub fn sys_futex_wait(vaddr: u64, expected: u32, timeout_ms: u64) -> i64 {
    let caller = current_pid().unwrap_or(0);
    if caller == 0 {
        return ERRNO_PERM;
    }
    if vaddr == 0 || (vaddr & 0x3) != 0 {
        return ERRNO_INVAL;
    }
    // The wake count is read before the word: a waker on another CPU that
    // runs between the check below and the sleep bumps it, and the sleep then
    // does not happen, instead of lasting the full timeout.
    let token = crate::sched::wake_token(caller);
    let key = (tgid_of(caller), vaddr);
    /*
     * Queued before the word is read, as Linux futex_wait_setup reads it
     * with the waiter's bucket held. Read first, a waker that changed the
     * word just after the read and woke before the queueing found nobody,
     * bumped no wake count, and this wait then slept its whole timeout with
     * the word already changed: for a timed wait, up to a minute.
     */
    join(&mut FUTEX_QUEUE.lock(), key, caller);
    // Re-read the word under the kernel's eyes: if it already changed since
    // the caller's own check, a wake happened and we must not block.
    let mut buf = [0u8; 4];
    let read = crate::usercopy::copy_from_user(vaddr, &mut buf);
    if read.is_err() || u32::from_ne_bytes(buf) != expected {
        // Bound first, so the queue lock is dropped before the wake.
        let next = leave_early(&mut FUTEX_QUEUE.lock(), key, caller);
        if let Some(next) = next {
            crate::sched::wake_process(next);
        }
        return if read.is_err() { ERRNO_FAULT } else { 0 };
    }

    // A timed wait sleeps the time it asked for: the queueing and the wake
    // token above mean a wake that races the sleep is never lost, so the
    // safety cap is only for an untimed wait. Capped, `mk_idle_ms` slept at most 20 ms, and every
    // driver loop counting sleeps (a stick's spin-up, a port's reset
    // recovery) ended long before the time it meant to give.
    let cap = if timeout_ms == 0 { SAFETY_MS } else { timeout_ms.min(MAX_TIMED_MS) };
    let deadline = crate::time::timestamp_millis().saturating_add(cap.max(1));
    crate::sched::sleep_until_unless_woken(caller, deadline, token);
    crate::sched::yield_now();

    // Woken by a waker or the safety deadline. Drop our own entry if the
    // waker did not already remove it.
    leave(&mut FUTEX_QUEUE.lock(), key, caller);
    0
}
