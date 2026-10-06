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

//! One worker thread at a time, on a stack that is mapped once and kept.
//!
//! An app that moves a slow call off its window thread needs one worker, not
//! many: a second request while one runs is refused, and the app says so. So
//! the stack is mapped on the first spawn, with its lowest page given back as
//! a guard (the kernel hands out ranges from a cursor that only moves up, so
//! nothing is mapped into that hole later and an overflow faults the thread
//! instead of writing over the heap), and every later worker runs on it.
//! Nothing is ever freed, so nothing can leak, and nothing is freed under a
//! thread still on it.
//!
//! A thread that has ended is gone from the process table only once the
//! kernel has finished its teardown; until then its stack may still be in
//! use, so the seat is busy until `mk_pid_alive` says the thread is gone.
//!
//! The kernel knows a worker by its own id. A service that compares the
//! caller's id with a process it knows, or a kernel gate that asks whether
//! the caller owns an endpoint (the Wi-Fi drivers are held to the Settings
//! endpoint that way), sees the thread, not the app, and refuses it.

use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

use super::entry::{Start, START_GAP};
use super::spawn::mk_thread_spawn;
use crate::mem::{mk_mmap, mk_munmap};
use crate::process::mk_pid_alive;

/// A worker's stack. Its job is one IPC call and what decodes the answer;
/// the buffers of that live on the heap.
pub const WORKER_STACK: usize = 64 * 1024;

const PAGE: usize = 4096;
/// Read and write; private and anonymous, which the microkernel ignores.
const PROT: i32 = 0x1 | 0x2;
const FLAGS: i32 = 0x02 | 0x20;
const USERSPACE_MAX: usize = 0x0000_7FFF_FFFF_FFFF;

const ENOMEM: i64 = -12;
const EBUSY: i64 = -16;

/// The seat is taken by a spawn that has not yet learned its thread's id.
const STARTING: u32 = u32::MAX;

pub struct WorkerSeat {
    /// The thread on the seat, 0 when none, `STARTING` during a spawn.
    tid: AtomicU32,
    /// The lowest byte of the stack above its guard page, 0 until mapped.
    stack: AtomicUsize,
}

impl Default for WorkerSeat {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkerSeat {
    pub const fn new() -> Self {
        Self { tid: AtomicU32::new(0), stack: AtomicUsize::new(0) }
    }

    /// Whether a thread is still on the seat. A worker that has handed its
    /// result back may still be on its way out; the seat is free only once
    /// the kernel has taken the thread out of the process table.
    pub fn busy(&self) -> bool {
        match self.tid.load(Ordering::Acquire) {
            0 => false,
            STARTING => true,
            tid if mk_pid_alive(tid) => true,
            tid => {
                // Gone. A spawn cannot have taken the seat meanwhile: it
                // only takes a seat that reads 0.
                let _ = self.tid.compare_exchange(tid, 0, Ordering::AcqRel, Ordering::Acquire);
                false
            }
        }
    }

    /// Run `job(arg)` on a new thread of this process. The thread's id, or a
    /// negative errno: EBUSY while the last worker is still on the seat,
    /// ENOMEM when its stack could not be mapped, or the kernel's refusal.
    /// `arg` is whatever the job needs, typically the address of a static.
    pub fn spawn(&self, job: fn(usize), arg: usize) -> Result<u32, i64> {
        if self.busy()
            || self.tid.compare_exchange(0, STARTING, Ordering::AcqRel, Ordering::Acquire).is_err()
        {
            return Err(EBUSY);
        }
        let Some(base) = self.stack_base() else {
            self.tid.store(0, Ordering::Release);
            return Err(ENOMEM);
        };
        let top = base + WORKER_STACK;
        let at = top - core::mem::size_of::<Start>().next_multiple_of(16);
        // SAFETY: `at` lies inside this seat's stack, which is mapped
        // writable and which no thread is on (the seat was free).
        unsafe { core::ptr::write(at as *mut Start, Start { job, arg }) };
        let rc = start_thread(at - START_GAP);
        if rc <= 0 {
            self.tid.store(0, Ordering::Release);
            return Err(if rc == 0 { ENOMEM } else { rc });
        }
        self.tid.store(rc as u32, Ordering::Release);
        Ok(rc as u32)
    }

    /// The stack, mapped on first use and kept.
    fn stack_base(&self) -> Option<usize> {
        let base = self.stack.load(Ordering::Acquire);
        if base != 0 {
            return Some(base);
        }
        let p = mk_mmap(core::ptr::null_mut(), WORKER_STACK + PAGE, PROT, FLAGS, -1, 0);
        if p.is_null() || (p as i64) < 0 || p as usize > USERSPACE_MAX {
            return None;
        }
        // The guard: an overflow faults here rather than writing below.
        // Should the kernel refuse the unmap the stack still works, unguarded.
        let _ = mk_munmap(p, PAGE);
        let base = p as usize + PAGE;
        self.stack.store(base, Ordering::Release);
        Some(base)
    }
}

#[cfg(target_arch = "x86_64")]
fn start_thread(stack: usize) -> i64 {
    mk_thread_spawn(super::entry::trampoline as *const () as usize as u64, stack as u64)
}

#[cfg(not(target_arch = "x86_64"))]
fn start_thread(_stack: usize) -> i64 {
    // No trampoline for this architecture yet: ENOSYS, and no thread.
    let _ = mk_thread_spawn;
    -38
}
