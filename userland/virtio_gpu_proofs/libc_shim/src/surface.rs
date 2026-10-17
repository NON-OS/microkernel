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
//! The surface registry, answering every register and share.

pub const SURFACE_FORMAT_ARGB8888: u32 = 1;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SurfaceDescriptor {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
    pub byte_len: u64,
    pub base_va: u64,
    pub flags: u64,
}

pub fn mk_surface_register(_desc: *const SurfaceDescriptor) -> i64 {
    1
}

pub fn mk_surface_share(sid: u64) -> i64 {
    0x5000 + sid as i64
}
