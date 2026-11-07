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

//! Anonymous pages, as the kernel maps them: zeroed and page aligned. Each
//! map and unmap is kept in the world, and an unmap of pages a registered
//! surface still names is counted, which the kernel would orphan.

use std::alloc::{alloc_zeroed, dealloc, Layout};

use super::world::with;

const PAGE: usize = 4096;

pub fn map(len: usize) -> i64 {
    let Ok(layout) = Layout::from_size_align(len, PAGE) else { return -22 };
    if len == 0 {
        return -22;
    }
    // SAFETY: the layout has a non-zero size.
    let at = unsafe { alloc_zeroed(layout) } as usize;
    if at == 0 {
        return -12;
    }
    with(|w| w.maps.push((at, len)));
    at as i64
}

pub fn unmap(at: usize, len: usize) -> i64 {
    let whole = with(|w| {
        let i = w.maps.iter().position(|&m| m == (at, len))?;
        w.maps.swap_remove(i);
        w.unmaps.push((at, len));
        let end = at + len;
        let named = |d: &super::world::SurfaceDescriptor| {
            let base = d.base_va as usize;
            base < end && at < base + d.byte_len as usize
        };
        if w.surfaces.iter().flatten().any(named) {
            w.unmapped_registered += 1;
        }
        Some(())
    });
    let Some(()) = whole else { return -22 };
    let Ok(layout) = Layout::from_size_align(len, PAGE) else { return -22 };
    // SAFETY: (at, len) was a live mapping `map` made with this layout, and
    // it has just been taken off the list, so it is freed once.
    unsafe { dealloc(at as *mut u8, layout) };
    0
}
