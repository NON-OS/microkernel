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

//! A table write made visible to a unit that reads memory without snooping
//! the CPU's caches. Control.Coherent asks the unit to snoop its
//! device table fetches; the line is flushed after every write as well, so
//! a unit that ignores the bit still reads the entry just written.

use core::arch::x86_64::{_mm_clflush, _mm_mfence};

/// Write one table entry, then push its line out to memory.
pub(crate) fn store(slot: &mut u64, value: u64) {
    *slot = value;
    flush(slot);
}

/// Push the cache line holding `at` out to memory, ordered before any
/// command that follows.
pub(crate) fn flush(at: &u64) {
    // SAFETY: eK@nonos.systems - `at` is a live reference, so the line is
    // mapped; CLFLUSH only writes it back and MFENCE only orders, and both
    // are in every x86_64 CPU (SSE2).
    unsafe {
        _mm_clflush((at as *const u64).cast());
        _mm_mfence();
    }
}
