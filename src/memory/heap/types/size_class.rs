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
//! The heap's front for small blocks: fixed size classes carved from slabs
//! the first-fit lists hand out, kept on free lists of their own. Pure, so
//! kernel_proofs holds it (heap_classes).
//!
//! The first-fit lists (linked_list_allocator) walk every hole below the one
//! that fits, on alloc and on free alike, with interrupts off and the region
//! lock held. Small blocks of many lifetimes, freed between large ones, are
//! what cut the lists into tens of thousands of holes; once they are served
//! here, a list holds only slabs and large blocks, and stays short. A class
//! is chosen from the layout alone, so a free finds the class its alloc did.
//! The free lists themselves are size_lists.rs.

/// The classes, in bytes. Each is a power of two, so a block at a class's
/// multiple inside a page-aligned slab is aligned to its class.
pub(crate) const CLASS_SIZES: [usize; 8] = [32, 64, 128, 256, 512, 1024, 2048, 4096];
pub(crate) const CLASSES: usize = CLASS_SIZES.len();
/// What one slab takes from a first-fit list, and its alignment.
pub(crate) const SLAB_BYTES: usize = 64 * 1024;
pub(crate) const SLAB_ALIGN: usize = 4096;

/// The class a block of `size` bytes aligned to `align` is served from, or
/// None when it goes to the first-fit lists.
pub(crate) const fn class_of(size: usize, align: usize) -> Option<usize> {
    if size == 0 || align == 0 || align > SLAB_ALIGN {
        return None;
    }
    let mut i = 0;
    while i < CLASSES {
        let class = CLASS_SIZES[i];
        if size <= class && align <= class {
            return Some(i);
        }
        i += 1;
    }
    None
}

pub(crate) use super::size_lists::SizeClasses;
