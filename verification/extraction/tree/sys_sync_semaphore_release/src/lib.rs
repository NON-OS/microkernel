// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/sys/sync/semaphore/release.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod sys;

pub fn semaphore_release(this: crate::sys::sync::semaphore::state::Semaphore) {
    this.release()
}

pub fn semaphore_available(this: crate::sys::sync::semaphore::state::Semaphore) -> usize {
    this.available()
}

pub fn semaphore_capacity(this: crate::sys::sync::semaphore::state::Semaphore) -> usize {
    this.capacity()
}

