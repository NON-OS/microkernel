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

//! The surface registry, input kinds and memory as the shell asks for them:
//! anonymous memory from the host's heap, and no surface ever registered
//! (the shell's own surfaces are made by the test, not by the kernel).

pub const SURFACE_FORMAT_ARGB8888: u32 = 1;
pub const INPUT_KIND_KEY_DOWN: u16 = 0;
pub const INPUT_KIND_POINTER_ABS: u16 = 3;
pub const INPUT_KIND_WHEEL: u16 = 4;
pub const INPUT_KIND_BUTTON_DOWN: u16 = 5;
pub const INPUT_KIND_BUTTON_UP: u16 = 6;
pub const INPUT_KIND_TOUCH: u16 = 7;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SurfaceDescriptor {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
    pub byte_len: u64,
    pub base_va: u64,
    pub flags: u64,
    pub owner_pid: u32,
    pub writable: u32,
}

pub fn mk_surface_register(_desc: *const SurfaceDescriptor) -> i64 {
    -38
}
pub fn mk_surface_release(_handle: u64) -> i64 {
    0
}
pub fn mk_surface_share(_sid: u64, _target_pid: u32, _flags: u64) -> i64 {
    -38
}

/// Zeroed pages from the host's heap, leaked: a test's frame lives as long.
pub fn mk_mmap(_: *mut u8, len: usize, _: i32, _: i32, _: i32, _: i64) -> *mut u8 {
    Box::leak(vec![0u8; len].into_boxed_slice()).as_mut_ptr()
}
pub fn mk_munmap(_addr: *mut u8, _len: usize) -> i64 {
    0
}
