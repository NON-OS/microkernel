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

//! The host's stand-in for the pages a window's pixels are mapped in
//! (capsule_linux wayland/scene_pixels_map.rs): zeroed, page aligned and
//! freed whole, with what is mapped now counted for the proofs.

use std::alloc::{alloc_zeroed, dealloc, Layout};
use std::cell::Cell;

const PAGE: usize = 4096;

thread_local! {
    static MAPPED: Cell<usize> = const { Cell::new(0) };
}

/// The bytes mapped now and not yet given back, on this thread.
pub fn mapped() -> usize {
    MAPPED.with(Cell::get)
}

pub fn map(len: usize) -> Option<usize> {
    let layout = Layout::from_size_align(len, PAGE).ok().filter(|l| l.size() != 0)?;
    // SAFETY: the layout has a non-zero size.
    let at = unsafe { alloc_zeroed(layout) } as usize;
    (at != 0).then(|| {
        MAPPED.with(|m| m.set(m.get() + len));
        at
    })
}

pub fn unmap(at: usize, len: usize) {
    let Ok(layout) = Layout::from_size_align(len, PAGE) else { return };
    MAPPED.with(|m| m.set(m.get() - len));
    // SAFETY: (at, len) is a mapping `map` made with this layout, given
    // back once, by the Pixels that held it.
    unsafe { dealloc(at as *mut u8, layout) };
}
