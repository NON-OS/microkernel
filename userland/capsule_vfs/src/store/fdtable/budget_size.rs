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
//! MkMmap backs every page of the heap when it is mapped, so vfs binds the
//! floor (plus HEAP_BESIDE for the staged packages) before it serves, and
//! grows the heap and the budget with it a GROW_PIECE at a time between
//! requests (server/grow.rs).

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

/// One growth step: 64 MiB, so the receive loop is held for one piece at
/// a time (16 384 pages backed), not for the budget whole.
pub const GROW_PIECE: usize = 64 << 20;

/// The next piece toward `wanted`, the budget being `budget` now.
pub fn next_piece(budget: usize, wanted: usize) -> Option<usize> {
    (budget < wanted).then(|| (wanted - budget).min(GROW_PIECE))
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
    fn the_budget_grows_from_the_floor_a_piece_at_a_time() {
        let wanted = for_ram(8 << 30);
        let (mut have, mut pieces) = (DATA_BYTES_MAX, 0);
        while let Some(p) = next_piece(have, wanted) {
            assert!(p <= GROW_PIECE && p > 0);
            have += p;
            pieces += 1;
        }
        assert_eq!((have, pieces), (wanted, (wanted - DATA_BYTES_MAX).div_ceil(GROW_PIECE)));
        assert_eq!(
            next_piece(DATA_BYTES_MAX, DATA_BYTES_MAX),
            None,
            "a small machine binds it all at once"
        );
        assert_eq!(next_piece(wanted - 1, wanted), Some(1));
    }
}
