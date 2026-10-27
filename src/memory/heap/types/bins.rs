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

//! Small blocks kept by size in front of the regions' hole lists
//! (region_alloc.rs). A hole list is walked from its head on every
//! allocation and every free, with the heap's lock held and interrupts off;
//! a long one, from many small blocks freed among live ones, stalled a large
//! install for minutes under emulation. A small block freed goes on its
//! class's list, and the next request of that class takes it back, so the
//! common case walks nothing and leaves no hole behind.

use core::alloc::Layout;

/// The classes, each a block's whole size: data, header and canary.
pub(crate) const CLASSES: [usize; 7] = [64, 128, 256, 512, 1024, 2048, 4096];
/// Every class block is this aligned, which serves any request up to it.
pub(crate) const ALIGN: usize = 16;
/// The most bytes a class keeps; past it a freed block goes back to its region.
pub(crate) const KEEP: usize = 1 << 20;

/// The class serving `layout`, and the layout its blocks are made with.
pub(crate) fn class_of(layout: Layout) -> Option<(usize, Layout)> {
    let class = CLASSES.iter().position(|&c| layout.size() <= c && layout.align() <= ALIGN)?;
    Some((class, Layout::from_size_align(CLASSES[class], ALIGN).ok()?))
}

/// What a class had to give.
pub(crate) enum Taken {
    Block(*mut u8),
    Empty,
    /// A kept block's link pointed outside the heap: a write after free.
    /// The class's list was dropped, and its blocks with it.
    Broken,
}
