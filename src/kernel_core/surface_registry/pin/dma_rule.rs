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

/* A DMA grant's frames under a shared surface, pure: held, never
 * freed, while another process maps the surface. kernel_proofs::dma_surface. */

extern crate alloc;

use alloc::vec::Vec;

/// A surface whose owner gave it up while another process maps it: its
/// frames by page index, and those `reclaim` frees one page at a time.
pub(crate) struct PageOrphan<H, F> {
    pub(crate) owner: u32,
    pub(crate) handle: H,
    pub(crate) frames: Vec<F>,
    pub(crate) free: Vec<F>,
}

/// A grant's frames held for the surfaces over them: one per grant, so it
/// is given back once however many surfaces it backs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct DmaOrphan<H> {
    /// Each surface over the grant, with its owner.
    pub(crate) handles: Vec<(H, u32)>,
    pub(crate) start: u64,
    pub(crate) len: u64,
}

/// Whether any page in `frames` lies in the grant `[start, start + len)`.
pub(crate) fn overlaps(frames: impl IntoIterator<Item = u64>, start: u64, len: u64) -> bool {
    let end = start.saturating_add(len);
    frames.into_iter().any(|f| start <= f && f < end)
}

/// The grant's frames taken out of a page orphan's free list, by `page`:
/// they are the broker's, freed through its pool, never page by page.
pub(crate) fn strip<T>(free: &mut Vec<T>, page: impl Fn(&T) -> u64, start: u64, len: u64) {
    let end = start.saturating_add(len);
    free.retain(|f| !(start <= page(f) && page(f) < end));
}

/// The held grants no process but their owner maps through any of their
/// surfaces, taken out of `held` to be given back, each exactly once.
/// `foreign(handle, owner)`: whether another process attaches `handle`.
pub(crate) fn due<H: Copy>(
    held: &mut Vec<DmaOrphan<H>>,
    foreign: impl Fn(H, u32) -> bool,
) -> Vec<DmaOrphan<H>> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < held.len() {
        if held[i].handles.iter().any(|&(h, owner)| foreign(h, owner)) {
            i += 1;
        } else {
            out.push(held.swap_remove(i));
        }
    }
    out
}
