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

//! The answer a worker thread hands to the window thread.
//!
//! A service whose reply is the result itself has to be waited for in one
//! call, since the kernel drops a reply that comes after its call gave up. A
//! worker waits instead of the window: the window thread claims the slot and
//! starts the worker (`WorkerSeat`), and on each tick looks whether the
//! answer is in. The worker puts it here and ends. The window side may stop
//! waiting (the Terminal's Ctrl+C): the worker still finishes its call, and
//! its answer is dropped where it lands, so a late answer never turns up
//! under a later request. Until then the slot stays taken and a new request
//! is refused. Kept in a static, the slot outlives the window: a worker that
//! finishes after its app closed writes into memory that is still there.
//!
//! Phases, and who moves them:
//!
//! - Idle: no request. The window thread claims it (Running).
//! - Running: the worker is on it. The worker puts its answer (Done), or the
//!   window thread stops waiting (Dropped).
//! - Done: the answer is in. The window thread takes it (Idle).
//! - Dropped: nobody waits. The worker's answer is discarded (Idle).
//!
//! The value is touched only by whoever the phase gives it to: the worker
//! while Running or Dropped, the window thread while Done.

use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicU8, Ordering};

const IDLE: u8 = 0;
const RUNNING: u8 = 1;
const DONE: u8 = 2;
const DROPPED: u8 = 3;

pub struct Handoff<T> {
    phase: AtomicU8,
    value: UnsafeCell<Option<T>>,
}

// SAFETY: the value moves between threads, so T must be Send; it is only
// ever reached by the one thread its phase gives it to (see above), with the
// phase stored Release after a write and loaded Acquire before a read.
unsafe impl<T: Send> Sync for Handoff<T> {}

/// What the window thread finds on its tick.
#[derive(Debug, PartialEq, Eq)]
pub enum Look<T> {
    /// No request is out.
    Idle,
    /// The worker is still on it, or a dropped one has not finished.
    Waiting,
    /// The answer, now the window thread's.
    Ready(T),
}

impl<T> Default for Handoff<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Handoff<T> {
    pub const fn new() -> Self {
        Self { phase: AtomicU8::new(IDLE), value: UnsafeCell::new(None) }
    }

    /// Window thread: take the slot for a new request. False while a worker
    /// is on it, including one whose answer nobody waits for any more.
    pub fn claim(&self) -> bool {
        self.phase.compare_exchange(IDLE, RUNNING, Ordering::AcqRel, Ordering::Acquire).is_ok()
    }

    /// Window thread: give back a claim whose worker never started.
    pub fn release(&self) {
        let _ = self.phase.compare_exchange(RUNNING, IDLE, Ordering::AcqRel, Ordering::Acquire);
    }

    /// Worker: hand the answer in, once, as the last thing it does here.
    pub fn put(&self, answer: T) {
        // SAFETY: the phase is Running or Dropped, in both of which only the
        // worker reaches the value.
        unsafe { *self.value.get() = Some(answer) };
        if self.phase.compare_exchange(RUNNING, DONE, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            return;
        }
        // Dropped: nobody will take it. Discard it here and free the slot.
        // SAFETY: still Dropped, so still only the worker's.
        let stale = unsafe { (*self.value.get()).take() };
        drop(stale);
        self.phase.store(IDLE, Ordering::Release);
    }

    /// Window thread: look whether the answer is in, and take it if it is.
    pub fn poll(&self) -> Look<T> {
        match self.phase.load(Ordering::Acquire) {
            DONE => {
                // SAFETY: Done gives the value to the window thread.
                let answer = unsafe { (*self.value.get()).take() };
                self.phase.store(IDLE, Ordering::Release);
                match answer {
                    Some(answer) => Look::Ready(answer),
                    None => Look::Idle,
                }
            }
            RUNNING | DROPPED => Look::Waiting,
            _ => Look::Idle,
        }
    }

    /// Window thread: stop waiting. A worker still on it finishes and its
    /// answer is discarded; an answer already in is discarded now.
    pub fn stop_waiting(&self) {
        if self
            .phase
            .compare_exchange(RUNNING, DROPPED, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            return;
        }
        if let Look::Ready(answer) = self.poll() {
            drop(answer);
        }
    }
}
