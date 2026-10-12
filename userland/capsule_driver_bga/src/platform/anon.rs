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
use nonos_gfx_serve::Region;
use nonos_gfx_wire::errno::ENOMEM;
use nonos_gfx_wire::Errno;
use nonos_libc::{mk_mmap, mk_munmap};

const PAGE: usize = 4096;
const PROT_READ_WRITE: i32 = 0x3;
const MAP_PRIVATE_ANON: i32 = 0x22;

/// Zeroed so a fresh primary never shows a previous owner's pixels.
pub fn map(len: usize) -> Result<Region, Errno> {
    let len = len.next_multiple_of(PAGE);
    let base = mk_mmap(core::ptr::null_mut(), len, PROT_READ_WRITE, MAP_PRIVATE_ANON, -1, 0);
    if (base as isize) <= 0 {
        return Err(ENOMEM);
    }
    // SAFETY: the mapping is this capsule's, `len` bytes, just made.
    unsafe { core::ptr::write_bytes(base, 0, len) };
    Ok(Region { base: base as usize, len })
}

pub fn unmap(r: Region) {
    let _ = mk_munmap(r.base as *mut u8, r.len);
}

pub fn bytes(r: Region) -> &'static [u8] {
    // SAFETY: regions are only those `map` made; the server drops its view
    // before it releases the region, so the slice never outlives the pages.
    unsafe { core::slice::from_raw_parts(r.base as *const u8, r.len) }
}
