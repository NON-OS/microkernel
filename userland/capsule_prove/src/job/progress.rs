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

//! What the worker tells the serving thread (worker.rs): its stage, and its
//! result once, published by the Release store of a final stage. The stage
//! is read with Acquire and the result only after a final one, so a result
//! is never read half-written and a STATUS mid-proof is answered at once.

use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicU32, Ordering};

use nonos_proof_abi::wire::Stage;

pub struct Progress<T> {
    stage: AtomicU32,
    result: UnsafeCell<Option<T>>,
}

// SAFETY: the worker writes the result before its final Release store; the
// serving thread reads it only after its Acquire load sees that store.
unsafe impl<T: Send> Sync for Progress<T> {}

impl<T> Default for Progress<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Progress<T> {
    pub const fn new() -> Self {
        Self { stage: AtomicU32::new(Stage::Queued as u32), result: UnsafeCell::new(None) }
    }

    /// Worker: the stage it starts now.
    pub fn advance(&self, stage: Stage) {
        self.stage.store(stage as u32, Ordering::Release);
    }

    /// Worker, once, as its last act here: the result, then the final stage.
    pub fn finish(&self, done: bool, result: T) {
        // SAFETY: no final stage is published yet, so the serving thread
        // does not read the result; only this worker writes it, once.
        unsafe { *self.result.get() = Some(result) };
        let last = if done { Stage::Done as u32 } else { Stage::Failed as u32 };
        self.stage.store(last, Ordering::Release);
    }

    /// The stage the worker last published.
    pub fn stage(&self) -> Stage {
        let v = u8::try_from(self.stage.load(Ordering::Acquire)).ok();
        v.and_then(Stage::from_u8).unwrap_or(Stage::Failed)
    }

    /// Serving thread: the result, once, when a final stage is published.
    pub fn take(&self) -> Option<T> {
        match self.stage() {
            // SAFETY: a final stage, stored after the result, was seen; it is done.
            Stage::Done | Stage::Failed => unsafe { (*self.result.get()).take() },
            _ => None,
        }
    }
}
