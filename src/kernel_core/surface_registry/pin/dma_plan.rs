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

/*
 * What the registry does with a DMA grant being released (K15), pure:
 * dma_hold.rs runs it under GATE, kernel_proofs::dma_surface on the host.
 */

extern crate alloc;

use alloc::vec::Vec;

use super::dma_rule::{overlaps, strip, DmaOrphan, PageOrphan};

/// `owner`'s grant `[start, start + len)` is released. `live`: its
/// surfaces over the grant, their frames already taken from their slots so
/// none can be attached again. Each one another process attaches becomes a
/// page orphan with nothing to free per page; every orphan of `owner` over
/// the grant loses the grant's frames from its free list. Some: the grant
/// is held for the surfaces still attached elsewhere. None: the caller frees.
pub(crate) fn hold_in<H: Copy, F>(
    (owner, start, len): (u32, u64, u64),
    live: Vec<(H, Vec<F>)>,
    orphans: &mut Vec<PageOrphan<H, F>>,
    foreign: impl Fn(H, u32) -> bool,
    page: impl Fn(&F) -> u64 + Copy,
) -> Option<DmaOrphan<H>> {
    for (handle, frames) in live {
        if foreign(handle, owner) {
            orphans.push(PageOrphan { owner, handle, frames, free: Vec::new() });
        }
    }
    let mut handles = Vec::new();
    for o in orphans.iter_mut().filter(|o| o.owner == owner) {
        if overlaps(o.frames.iter().map(page), start, len) {
            strip(&mut o.free, page, start, len);
            if foreign(o.handle, owner) {
                handles.push(o.handle);
            }
        }
    }
    (!handles.is_empty()).then_some(DmaOrphan { owner, handles, start, len })
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
        if held[i].handles.iter().any(|&h| foreign(h, held[i].owner)) {
            i += 1;
        } else {
            out.push(held.swap_remove(i));
        }
    }
    out
}
