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

//! The shadow: memory the compositor draws into, shared as a surface. The
//! adapter has no DMA, so a present copies the damaged rect into VRAM.

use nonos_libc::{
    mk_mmap, mk_munmap, mk_surface_register, mk_surface_release, mk_surface_share,
    SurfaceDescriptor, SURFACE_FORMAT_ARGB8888,
};

use crate::dispi::Mode;
use crate::error::{BgaError, BgaResult};

const PROT_READ_WRITE: i32 = 0x3;
const MAP_PRIVATE_ANON: i32 = 0x22;

pub struct Shadow {
    pub va: u64,
    pub len: usize,
    /// What the compositor attaches; 0 until shared.
    pub handle: u64,
}

impl Shadow {
    pub fn new(m: Mode) -> BgaResult<Self> {
        let len = usize::try_from(m.bytes()).map_err(|_| BgaError::Surface(-12))?;
        let p = mk_mmap(core::ptr::null_mut(), len, PROT_READ_WRITE, MAP_PRIVATE_ANON, -1, 0);
        if p.is_null() || (p as i64) < 0 {
            return Err(BgaError::Surface(p as i64));
        }
        let mut s = Shadow { va: p as u64, len, handle: 0 };
        let desc = SurfaceDescriptor {
            width: m.width,
            height: m.height,
            stride: m.stride,
            format: SURFACE_FORMAT_ARGB8888,
            byte_len: m.bytes(),
            base_va: s.va,
            flags: 0,
        };
        let sid = mk_surface_register(&desc);
        if sid < 0 {
            return Err(BgaError::Surface(sid));
        }
        let handle = mk_surface_share(sid as u64);
        if handle <= 0 {
            return Err(BgaError::Surface(handle));
        }
        s.handle = handle as u64;
        Ok(s)
    }
}

impl Drop for Shadow {
    fn drop(&mut self) {
        if self.handle != 0 {
            let _ = mk_surface_release(self.handle);
        }
        let _ = mk_munmap(self.va as *mut u8, self.len);
    }
}
