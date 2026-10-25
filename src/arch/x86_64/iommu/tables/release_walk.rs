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

//! The order a dead domain's tables go back in: every table below the root,
//! deepest first, then the root. A leaf (level 1, or a large page) points at
//! a frame the broker granted, which the broker scrubs and frees itself, so
//! the walk never follows one. Pure over `child`, `read` and `free`, so VT-d
//! and AMD-Vi share it and the host proofs drive it against a model.

use super::sl_pte::{entry_address, is_present, ENTRIES, SL_LARGE};

/// The VT-d rule: the table an entry at `level` leads to, or `None` for a
/// leaf or an empty entry.
pub const fn child_table(entry: u64, level: u8) -> Option<u64> {
    if level <= 1 || !is_present(entry) || entry & SL_LARGE != 0 {
        None
    } else {
        Some(entry_address(entry))
    }
}

/// Free `table` (at `level`) and every table under it, children first.
/// Returns how many were freed. The first error stops the walk: what was
/// not freed by then stays allocated, never reused, which is the safe way to
/// be wrong.
pub fn release_walk<E>(
    table: u64,
    level: u8,
    child: &impl Fn(u64, u8) -> Option<u64>,
    read: &impl Fn(u64, usize) -> Result<u64, E>,
    free: &mut impl FnMut(u64) -> Result<(), E>,
) -> Result<usize, E> {
    let mut freed = 0;
    for index in 0..ENTRIES {
        if let Some(next) = child(read(table, index)?, level) {
            freed += release_walk(next, level - 1, child, read, free)?;
        }
    }
    free(table)?;
    Ok(freed + 1)
}
