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

//! The memory behind the surface: an anonymous mapping of its own, so the
//! kernel's surface entry names exactly it, unmapped when it drops, on a
//! failed step and with the driver. Unmapping it all gives up the kernel's
//! entry too, or leaves the frames to the kernel until the compositor lets
//! go of them.

use nonos_libc::{mk_mmap, mk_munmap};

use crate::error::{BgaError, BgaResult};

const PROT_READ_WRITE: i32 = 0x3;
const MAP_PRIVATE_ANON: i32 = 0x22;

pub struct Memory {
    pub va: u64,
    len: usize,
}

pub fn map(len: usize) -> BgaResult<Memory> {
    let va = mk_mmap(core::ptr::null_mut(), len, PROT_READ_WRITE, MAP_PRIVATE_ANON, -1, 0) as u64;
    if va as i64 <= 0 {
        return Err(BgaError::SurfaceRefused);
    }
    Ok(Memory { va, len })
}

impl Drop for Memory {
    fn drop(&mut self) {
        let _ = mk_munmap(self.va as *mut u8, self.len);
    }
}
