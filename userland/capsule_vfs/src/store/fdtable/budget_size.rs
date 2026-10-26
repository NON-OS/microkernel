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

//! How much the clients' files may hold together on this machine.
//!
//! An eighth of its memory, never less than DATA_BYTES_MAX and never more
//! than DATA_BYTES_CEIL. MkMmap backs every page of the heap when it is
//! mapped, so the heap is sized from the same number (main.rs): this budget,
//! and HEAP_BESIDE for the packages staged at boot and the server's own.

use core::sync::atomic::{AtomicUsize, Ordering};

pub(crate) use super::budget::DATA_BYTES_MAX;

/// The most the clients' files hold, on any machine.
pub const DATA_BYTES_CEIL: usize = 512 << 20;
/// The heap beside the budget: the staged packages (MAX_TOTAL_BYTES, 96 MiB),
/// the names, handles and journal, and the request buffers.
pub const HEAP_BESIDE: usize = 160 << 20;

static LIMIT: AtomicUsize = AtomicUsize::new(DATA_BYTES_MAX);

/// The budget for a machine with `total` bytes of memory.
pub fn for_ram(total: u64) -> usize {
    usize::try_from(total / 8).unwrap_or(usize::MAX).clamp(DATA_BYTES_MAX, DATA_BYTES_CEIL)
}

/// The budget in force: DATA_BYTES_MAX until `set`.
pub fn limit() -> usize {
    LIMIT.load(Ordering::Relaxed)
}

/// Make `budget` the one in force, once a heap that holds it is mapped.
pub fn set(budget: usize) {
    LIMIT.store(budget, Ordering::Relaxed);
}

#[cfg(test)]
mod tests {
    use super::*;

    const GIB: u64 = 1 << 30;

    #[test]
    fn the_budget_is_an_eighth_of_memory_within_its_bounds() {
        assert_eq!(for_ram(GIB), DATA_BYTES_MAX);
        assert_eq!(for_ram(4 * GIB), 512 << 20);
        assert_eq!(for_ram(64 * GIB), DATA_BYTES_CEIL);
        assert_eq!(for_ram(0), DATA_BYTES_MAX);
    }
}
