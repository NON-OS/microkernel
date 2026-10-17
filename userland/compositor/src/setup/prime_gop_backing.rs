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

//! The GOP path's screen size and its primary surface's backing.

use nonos_libc::{mk_mmap, nonos_display_dimensions};

const PROT_READ_WRITE: i32 = 0x3;
const MAP_PRIVATE_ANON: i32 = 0x22;

pub(super) fn display_dimensions() -> Result<(u32, u32), &'static str> {
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    let rc = nonos_display_dimensions(0, &mut width as *mut u32, &mut height as *mut u32);
    if rc < 0 || width == 0 || height == 0 {
        return Err("gop: no display dimensions");
    }
    Ok((width, height))
}

// Back the surface with its own anonymous mapping, not the heap. mmap
// gives a dedicated, page-aligned VMA whose start is exactly base_va, so
// the kernel's present path resolves the surface to a real region. A
// heap allocation would sit inside the heap VMA and never match.
pub(super) fn alloc_backing(byte_len: u64) -> Result<u64, &'static str> {
    let size = usize::try_from(byte_len).map_err(|_| "gop: backing too large")?;
    let ptr = mk_mmap(core::ptr::null_mut(), size, PROT_READ_WRITE, MAP_PRIVATE_ANON, -1, 0);
    if ptr.is_null() {
        return Err("gop: backing mmap failed");
    }
    Ok(ptr as u64)
}
