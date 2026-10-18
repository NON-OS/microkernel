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
 * What the registry does with a DMA grant being released, pure:
 * dma_hold.rs runs it under GATE, kernel_proofs::dma_surface on the host.
 */

extern crate alloc;

use alloc::vec::Vec;

use super::dma_rule::{overlaps, strip, DmaOrphan, PageOrphan};

/// `grant`'s owner, `[start, start + len)`, is released. `same(a, b)`:
/// whether pids `a` and `b` are threads of one process, so a grant mapped
/// by a worker is matched to surfaces its main thread registered.
/// `live`: those surfaces over the grant with their owners, their frames
/// already taken from their slots so none can be attached again. Each one
/// another process attaches becomes a page orphan with nothing to free per
/// page; every orphan of the process over the grant loses the grant's
/// frames from its free list. Some: the grant is held for the surfaces
/// still attached elsewhere. None: the caller frees.
pub(crate) fn hold_in<H: Copy, F>(
    (grant, start, len): (u32, u64, u64),
    live: Vec<(H, u32, Vec<F>)>,
    orphans: &mut Vec<PageOrphan<H, F>>,
    (foreign, same): (impl Fn(H, u32) -> bool, impl Fn(u32, u32) -> bool),
    page: impl Fn(&F) -> u64 + Copy,
) -> Option<DmaOrphan<H>> {
    for (handle, owner, frames) in live {
        if foreign(handle, owner) {
            orphans.push(PageOrphan { owner, handle, frames, free: Vec::new() });
        }
    }
    let mut handles = Vec::new();
    for o in orphans.iter_mut().filter(|o| same(o.owner, grant)) {
        if overlaps(o.frames.iter().map(page), start, len) {
            strip(&mut o.free, page, start, len);
            if foreign(o.handle, o.owner) {
                handles.push((o.handle, o.owner));
            }
        }
    }
    (!handles.is_empty()).then_some(DmaOrphan { handles, start, len })
}
