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

//! A window's two buffers and the swapchain over them. The compositor reads
//! the one shown; the bridge draws only in the other, taken once the
//! compositor released it (nonos_surface Swap, MkSurfaceFenceWait), so a
//! guest's frame is never composed half laid. Each buffer is registered,
//! shared read only with the compositor and, when it scans out client
//! buffers, with the display driver it names.

use core::mem;

use nonos_app_skeleton::swap::{Damage, Pending, Stale, Swap};
use nonos_libc::mk_surface_release;

use super::reshape::Shape;
use super::scene_pixels::Pixels;
use super::swap_copy::{clip, copy, outside, Area};

/// Double buffering, as the project chose for every window.
pub const BUFFERS: usize = 2;

/// How long a draw waits for the compositor to release a buffer: two
/// refreshes at 60 Hz. One that finds none in that time is owed (owed.rs)
/// and the next waits for none until a buffer is free again.
const ACQUIRE_MS: u64 = 33;
const EINVAL: i64 = -22;

/// How long a draw may wait for a buffer.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Patience {
    /// Up to two refreshes, unless the last wait found none (ACQUIRE_MS).
    Fence,
    /// Not at all: a draw input made (a control lit under the pointer) is
    /// owed instead, so the serve loop goes on to the input behind it.
    Now,
}

pub struct Buffers {
    px: [Pixels; BUFFERS],
    handles: [u64; BUFFERS],
    /// The width, height and stride both are registered at.
    shape: Shape,
    swap: Option<Swap>,
    /// The last acquire found none free.
    starved: bool,
}

impl Default for Buffers {
    fn default() -> Buffers {
        Buffers::empty()
    }
}

impl Buffers {
    /// No buffers: a window not shown yet.
    pub const fn empty() -> Buffers {
        Buffers {
            px: [Pixels::empty(), Pixels::empty()],
            handles: [0; BUFFERS],
            shape: (0, 0, 0),
            swap: None,
            starved: false,
        }
    }

    /// Two buffers of `bytes`; None, when the heap has no room for both.
    pub fn room(bytes: usize) -> Option<[Pixels; BUFFERS]> {
        Some([Pixels::with_room(bytes)?, Pixels::with_room(bytes)?])
    }

    /// `px`, buffer 0 drawn, registered at `shape` and shared with each of
    /// `peers` (the compositor, then the scanout driver if any), as the
    /// layer's chain in band `z` with buffer 0 shown. All or none: a buffer
    /// that fails gives back the one before it (and a pair refused is let
    /// go of whole as it drops).
    pub fn register(
        px: [Pixels; BUFFERS],
        shape: Shape,
        peers: (u32, u32),
        z: u32,
        opaque: bool,
    ) -> Result<Buffers, i64> {
        let mut b = Buffers { px, handles: [0; BUFFERS], shape, swap: None, starved: false };
        let mut pairs = [(0u64, 0u64); BUFFERS];
        for (i, pair) in pairs.iter_mut().enumerate() {
            match super::present::surface::register(&b.px[i], shape, peers) {
                Ok(p) => (*pair, b.handles[i]) = (p, p.1),
                Err(rc) => return Err(rc),
            }
        }
        let mut swap = Swap::new(&pairs, z).ok_or(EINVAL)?;
        swap.set_opaque(opaque);
        if !swap.adopt(0) {
            return Err(EINVAL);
        }
        b.swap = Some(swap);
        Ok(b)
    }

    /// The handle the layer shows, which names it to the compositor.
    pub fn shown(&self) -> Option<u64> {
        self.swap.as_ref().map(Swap::shown_handle)
    }

    /// The first `bytes` of the buffer shown, which holds the picture.
    pub fn shown_frame(&self, bytes: usize) -> Option<&[u8]> {
        let shown = self.shown()?;
        let j = self.handles.iter().position(|&h| h == shown)?;
        self.px.get(j)?.frame_ref(bytes)
    }

    pub fn handle(&self, i: usize) -> u64 {
        self.handles.get(i).copied().unwrap_or(0)
    }

    /// Whether a frame of `bytes` fits in these buffers.
    pub fn fits(&self, bytes: usize) -> bool {
        self.swap.is_some() && self.px.iter().all(|p| p.span().1 >= bytes as u64)
    }

    pub fn shape(&self) -> Shape {
        self.shape
    }

    /// A buffer to draw in, brought up to the picture shown except `cover`,
    /// which the caller draws whole; None when the compositor released
    /// none in time. A draw that would not wait says nothing of whether
    /// the compositor is releasing buffers, so only one that waited marks
    /// the chain starved.
    pub fn acquire(&mut self, cover: Area, patience: Patience) -> Option<usize> {
        let wait = match patience {
            Patience::Fence if !self.starved => ACQUIRE_MS,
            Patience::Fence | Patience::Now => 0,
        };
        let taken = self.swap.as_mut()?.acquire(wait);
        if patience == Patience::Fence || taken.is_some() {
            self.starved = taken.is_none();
        }
        let (i, stale) = taken?;
        let stale = match stale {
            Stale::None => return Some(i),
            Stale::Box(d) => (d.x, d.y, d.width, d.height),
            Stale::Whole => (0, 0, self.shape.0, self.shape.1),
        };
        if self.bring_up(i, stale, cover) {
            return Some(i);
        }
        self.cancel(i);
        None
    }

    /// The parts of `stale` outside `cover` copied into buffer `i` from the
    /// one shown, which holds the whole picture.
    fn bring_up(&mut self, i: usize, stale: Area, cover: Area) -> bool {
        let Some(shown) = self.shown() else { return false };
        let Some(j) = self.handles.iter().position(|&h| h == shown) else { return false };
        if i == j {
            return true;
        }
        let bytes = self.shape.2 as usize * self.shape.1 as usize;
        let (lo, hi) = self.px.split_at_mut(i.max(j));
        let (to, from) = if i < j { (&mut lo[i], &hi[0]) } else { (&mut hi[0], &lo[j]) };
        let from = from.frame_ref(bytes);
        let (Some(to), Some(from)) = (to.frame(bytes), from) else { return false };
        let (w, h, _) = self.shape;
        outside(stale, cover).into_iter().flatten().all(|a| {
            let (_, _, bw, bh) = clip(a, (w, h)).unwrap_or_default();
            let copied = copy(to, from, self.shape, a);
            if copied {
                super::commit::launch_clock::laid(super::frame_cost::box_bytes(bw, bh));
            }
            copied
        })
    }

    /// Buffer `i`'s pixels, out of the pair while they are drawn.
    pub fn take(&mut self, i: usize) -> Pixels {
        self.px.get_mut(i).map(mem::take).unwrap_or_default()
    }

    pub fn put(&mut self, i: usize, px: Pixels) {
        if let Some(slot) = self.px.get_mut(i) {
            *slot = px;
        }
    }

    /// The present of buffer `i` with `damage`; None without a chain.
    pub fn present(&self, i: usize, damage: Area) -> Option<Pending> {
        let (x, y, width, height) = damage;
        Some(self.swap.as_ref()?.present(i, Damage { x, y, width, height }))
    }

    /// The compositor's answer: taken, buffer `i` is shown.
    pub fn settle(&mut self, p: Pending, taken: bool) -> bool {
        self.swap.as_mut().is_some_and(|s| s.settle(p, taken).is_ok()) && taken
    }

    /// Buffer `i` given back undrawn.
    pub fn cancel(&mut self, i: usize) {
        if let Some(s) = self.swap.as_mut() {
            s.cancel(i);
        }
    }

    /// Each surface still registered released, and the pixels of one the
    /// kernel would not let go of kept for good, since it may still map
    /// them. True when every one was let go of.
    pub fn release(&mut self) -> bool {
        self.swap = None;
        let mut all = true;
        for (h, px) in self.handles.iter_mut().zip(self.px.iter_mut()) {
            let handle = mem::take(h);
            if handle != 0 && mk_surface_release(handle) < 0 {
                mem::forget(mem::take(px));
                all = false;
            }
        }
        all
    }
}

/// A pair dropped is let go of first: no pixels a surface still names go
/// back to the heap.
impl Drop for Buffers {
    fn drop(&mut self) {
        let _ = self.release();
    }
}
