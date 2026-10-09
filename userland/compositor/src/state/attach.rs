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

//! The surfaces the compositor has mapped, by handle.
//!
//! A mapping is trusted only while the kernel still knows the surface. When
//! a window's owner exits without a scene remove (a crash, a kill, a guest
//! that ends), the kernel drops the surface's slot and keeps its frames as
//! an orphan until every other holder releases them. The cache used to
//! answer from its own copy of the mapping for good: the dead window was
//! never reaped, it stayed drawn, and (before the kernel kept such frames)
//! every composite over it read pages handed to someone else, which showed
//! as tiles of other pictures where the window had been. Each lookup now
//! asks the kernel first (an attach the kernel already holds is answered
//! from its record, with no new mapping and no new reference); a handle
//! whose surface is gone is released at once, which frees the orphan, and
//! reported gone so its layer goes too.

use crate::sw_blitter::Surface;

pub const MAX_ATTACH: usize = 32;
const EMPTY: Surface = Surface { base_va: 0, stride: 0, width: 0, height: 0, byte_len: 0 };

/// The two surface calls the cache makes, so the host proofs can stand a
/// kernel in for the real one.
pub trait SurfaceKernel {
    /// Map `handle` (or find the mapping already held), or `None` when the
    /// kernel has no such surface.
    fn attach(&mut self, handle: u64) -> Option<Surface>;
    /// Drop this process's view of and reference to `handle`; false when the
    /// kernel refused.
    fn release(&mut self, handle: u64) -> bool;
}

/// What a lookup found.
#[derive(Clone, Copy)]
pub enum Attached {
    /// The surface is mapped; draw from it.
    Live(Surface),
    /// It was mapped here and its owner is gone: the mapping has been let
    /// go (the kernel frees the frames on that release) and the layer must
    /// be dropped now.
    Gone,
    /// It never attached. It may yet (the miss count decides).
    Missing,
    /// Every slot holds a surface a layer of the scene is drawn from, so
    /// this one was not mapped: nothing is mapped that is not tracked.
    NoRoom,
}

#[derive(Clone, Copy, Default)]
struct Slot {
    handle: u64,
    surface: Surface,
    in_use: bool,
    /// When this slot last answered a lookup, for eviction.
    used: u64,
}

pub struct AttachCache {
    slots: [Slot; MAX_ATTACH],
    clock: u64,
}

impl AttachCache {
    pub const fn new() -> Self {
        Self {
            slots: [Slot { handle: 0, surface: EMPTY, in_use: false, used: 0 }; MAX_ATTACH],
            clock: 0,
        }
    }

    /// Look `handle` up for a composite. `in_scene` holds the handles the
    /// scene's layers are drawn from. A full cache makes room by letting go
    /// of the least recently used mapping no layer is drawn from, and maps
    /// nothing when there is none. It used to map such a surface afresh on
    /// every composite without keeping it, a mapping leaked each frame.
    pub fn lookup(
        &mut self,
        handle: u64,
        in_scene: &[u64],
        kernel: &mut impl SurfaceKernel,
    ) -> Attached {
        if handle == 0 {
            return Attached::Missing;
        }
        self.clock += 1;
        if let Some(i) = self.slots.iter().position(|s| s.in_use && s.handle == handle) {
            let Some(surface) = kernel.attach(handle) else {
                // The owner exited. The kernel keeps this process's attach
                // record, and the orphaned frames, until it is released: let
                // go now, or the window's memory is never freed.
                let _ = kernel.release(handle);
                self.slots[i] = Slot::default();
                return Attached::Gone;
            };
            self.slots[i].surface = surface;
            self.slots[i].used = self.clock;
            return Attached::Live(surface);
        }
        let Some(free) = self.room(in_scene, kernel) else {
            return Attached::NoRoom;
        };
        let Some(surface) = kernel.attach(handle) else {
            return Attached::Missing;
        };
        self.slots[free] = Slot { handle, surface, in_use: true, used: self.clock };
        Attached::Live(surface)
    }

    /// A free slot, or one made free by letting go of the least recently
    /// used mapping that no layer in `in_scene` is drawn from.
    fn room(&mut self, in_scene: &[u64], kernel: &mut impl SurfaceKernel) -> Option<usize> {
        if let Some(i) = self.slots.iter().position(|s| !s.in_use) {
            return Some(i);
        }
        let i = (0..MAX_ATTACH)
            .filter(|&i| !in_scene.contains(&self.slots[i].handle))
            .min_by_key(|&i| self.slots[i].used)?;
        let _ = kernel.release(self.slots[i].handle);
        self.slots[i] = Slot::default();
        Some(i)
    }

    /// Let go of `handle`; false when the kernel refused the release. The
    /// slot goes either way: a refusal means this process holds no reference
    /// any more, and a slot kept for it would serve a dead mapping the next
    /// time the handle number came round.
    pub fn forget(&mut self, handle: u64, kernel: &mut impl SurfaceKernel) -> bool {
        let mut released = true;
        for slot in self.slots.iter_mut() {
            if slot.in_use && slot.handle == handle {
                released &= kernel.release(handle);
                *slot = Slot::default();
            }
        }
        released
    }
}

impl Default for AttachCache {
    fn default() -> Self {
        Self::new()
    }
}
