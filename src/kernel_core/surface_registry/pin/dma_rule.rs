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

/* A DMA run's frames are the broker's (K15, L35): exit_owned.rs takes them
 * off an orphan's per-page free list. Pure: kernel_proofs::dma_surface. */

extern crate alloc;

use alloc::vec::Vec;

/// The grant's frames taken out of a page orphan's free list, by `page`:
/// they are the broker's, freed through its pool, never page by page.
pub(crate) fn strip<T>(free: &mut Vec<T>, page: impl Fn(&T) -> u64, start: u64, len: u64) {
    let end = start.saturating_add(len);
    free.retain(|f| !(start <= page(f) && page(f) < end));
}
