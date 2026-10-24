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

//! What a panic can still wipe.
//!
//! A panic stops the machine with every byte it held still in DRAM, and
//! `terminate`'s wipe cannot run here: it takes the process table, the pager
//! and the allocator, and the panic may have been raised holding any of them.
//! This runs only what takes no lock and allocates nothing, after the other
//! CPUs are stopped and before the halt: the kernel heap, by the extent the
//! allocator keeps in two atomics, and the RAM log if nobody holds it.
//!
//! The heap is where the key vault's map, IPC payloads, the file caches and
//! the session volume's key live, so they go with it. Process memory is not
//! reached: its frames come from the page allocator, and finding them means
//! the locks above (docs/security/privacy-guarantees.md says what that leaves).

use crate::security::hardening::memory_sanitization::erase::secure_zero;

/// Zero what the panic path can reach without a lock. Called once, from the
/// panic handler, after everything it prints has been printed.
pub(crate) fn panic_wipe() {
    // The log's own lock is tried, never waited on: a panic raised while
    // logging holds it, and that log stays (serial already has its lines).
    let _ = crate::log::wipe_ram_log();
    if let Some((start, size)) = crate::memory::heap::get_allocator().extent() {
        // The extent is the allocator's whole window, set once at init; the
        // directmap maps it read-write and the other CPUs are halted.
        secure_zero(start, size);
    }
}
