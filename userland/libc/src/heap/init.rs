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

use core::sync::atomic::{AtomicBool, Ordering};

use super::allocator::ALLOCATOR;
use super::span;

const INITIAL_HEAP_SIZE: usize = 16 * 1024 * 1024;

static INITIALIZED: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HeapError {
    AlreadyInitialized,
    MmapFailed,
}

/// Bind the global allocator to the default 16 MiB region. One-shot: the
/// first call locks initialisation; subsequent calls return
/// `AlreadyInitialized`. On `mk_mmap` failure the flag is released for retry.
pub fn init() -> Result<(), HeapError> {
    init_sized(INITIAL_HEAP_SIZE)
}

/// Bind the global allocator to a region of `bytes` returned by `mk_mmap`,
/// in gigabyte pieces when it is larger (`span`). A memory-hungry capsule
/// (the browser, the prover) calls this from its entry point
/// with a larger size before the skeleton's default `init`, which then sees
/// `AlreadyInitialized` and proceeds. Keeping the larger heap opt-in means
/// the common capsule footprint stays at the 16 MiB default.
pub fn init_sized(bytes: usize) -> Result<(), HeapError> {
    if INITIALIZED.swap(true, Ordering::SeqCst) {
        return Err(HeapError::AlreadyInitialized);
    }
    let Some(base) = span::map(bytes) else {
        INITIALIZED.store(false, Ordering::SeqCst);
        return Err(HeapError::MmapFailed);
    };
    /* SAFETY: ek@nonos.systems: `span::map` returned userspace VA mapped
     * writable and contiguous, so `[base, base + bytes)` is this process's. */
    unsafe {
        ALLOCATOR.init(base, bytes);
    }
    Ok(())
}
