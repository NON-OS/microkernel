// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/sys/sync/semaphore/pure.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/sys/sync/semaphore/pure.rs"]
pub mod pure;

pub fn can_acquire(count: usize) -> bool {
    pure::can_acquire(count)
}

pub fn acquire_count(count: usize) -> usize {
    pure::acquire_count(count)
}

pub fn release_count(count: usize, cap: usize) -> usize {
    pure::release_count(count, cap)
}

