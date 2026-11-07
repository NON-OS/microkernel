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

//! A heap grown after it was bound: one fresh piece mapped and taken only
//! when it begins where the heap ends, then handed to the allocator. A
//! capsule that serves can bind the heap its first requests need and grow
//! toward the rest between requests, instead of holding its first answer
//! until every page of a large heap is backed (vfs: track 8b).

use core::sync::atomic::{AtomicUsize, Ordering};

use super::allocator::ALLOCATOR;
use super::init::HeapError;
use crate::mem::mk_munmap;

/// The heap's end, once bound; 0 before.
static END: AtomicUsize = AtomicUsize::new(0);

pub(super) fn bound(base: *mut u8, bytes: usize) {
    END.store(base as usize + bytes, Ordering::Release);
}

/// Grow the heap by `bytes`, at most `span::PIECE`. `NotContiguous` when the
/// kernel placed the piece anywhere but the heap's end (it is given back);
/// `MmapFailed` when there is no memory for it; `NotInitialized` before
/// the heap is bound.
pub fn grow(bytes: usize) -> Result<(), HeapError> {
    let end = END.load(Ordering::Acquire);
    if end == 0 {
        return Err(HeapError::NotInitialized);
    }
    if bytes == 0 || bytes > super::span::PIECE {
        return Err(HeapError::MmapFailed);
    }
    let Some(p) = super::span::piece(bytes) else { return Err(HeapError::MmapFailed) };
    if p as usize != end {
        mk_munmap(p, bytes);
        return Err(HeapError::NotContiguous);
    }
    // SAFETY: the piece was mapped writable for this process just now and
    // begins at the heap's end, so the heap's region grows by exactly it.
    unsafe { ALLOCATOR.extend(bytes) };
    END.store(end + bytes, Ordering::Release);
    super::stats::grew(bytes);
    Ok(())
}
