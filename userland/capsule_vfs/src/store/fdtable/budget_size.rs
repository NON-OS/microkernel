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
//! A quarter of its memory, never less than DATA_BYTES_MAX and never more
//! than DATA_BYTES_CEIL, so three quarters always stay with everything else.
//! MkMmap backs every page of the heap when it is mapped, so the heap is sized
//! from the same number (main.rs), plus HEAP_BESIDE for the staged packages.

use core::sync::atomic::{AtomicUsize, Ordering};

pub(crate) use super::budget::DATA_BYTES_MAX;

/// The most the clients' files hold, on any machine.
pub const DATA_BYTES_CEIL: usize = 4 << 30;
/// The heap beside the budget: the staged packages (MAX_TOTAL_BYTES, 96 MiB),
/// the names, handles and journal, and the request buffers.
pub const HEAP_BESIDE: usize = 160 << 20;

static LIMIT: AtomicUsize = AtomicUsize::new(DATA_BYTES_MAX);

/// The budget for a machine with `total` bytes of memory.
pub fn for_ram(total: u64) -> usize {
    usize::try_from(total / 4).unwrap_or(usize::MAX).clamp(DATA_BYTES_MAX, DATA_BYTES_CEIL)
}

/// The budgets to try in turn, halving, so a refused heap is not the floor.
pub fn steps(wanted: usize) -> impl Iterator<Item = usize> {
    let half = |b: &usize| (*b > DATA_BYTES_MAX).then(|| (b / 2).max(DATA_BYTES_MAX));
    core::iter::successors(Some(wanted.max(DATA_BYTES_MAX)), half)
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

    #[test]
    fn the_budget_is_a_quarter_of_memory_within_its_bounds() {
        let at = |g: u64| for_ram(g << 30) >> 20;
        assert_eq!([0, 1, 2, 4, 8, 16, 32].map(at), [160, 256, 512, 1024, 2048, 4096, 4096]);
    }

    #[test]
    fn a_refused_heap_halves_down_to_the_floor() {
        let mib: alloc::vec::Vec<usize> = steps(4 << 30).map(|b| b >> 20).collect();
        assert_eq!(mib, [4096, 2048, 1024, 512, 256, 160]);
        assert_eq!(steps(DATA_BYTES_MAX).count(), 1);
    }
}
