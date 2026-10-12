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

//! The firmware's boot logo (F-50): the BGRT image the firmware drew, so the
//! boot splash can show it where it was. `which` 0 asks for the 28-byte
//! record (x, y, width, height u32; flags u8, bit 0 on screen, bit 1 top-down;
//! quarter turns u8; bits per pixel u16; file size u32; pixel offset u32),
//! `which` 1 for the BMP file, already checked by the kernel. Cap requirement:
//! GraphicsSurfaceCreate, and only the process that owns `app.boot_splash`.

use crate::syscall::{call_raw, N_MK_BOOT_LOGO};

pub const BOOT_LOGO_RECORD: u32 = 0;
pub const BOOT_LOGO_FILE: u32 = 1;
pub const BOOT_LOGO_RECORD_LEN: usize = 28;

/// The bytes written; with a null `out`, the length the answer needs.
/// -ENOENT when the firmware left no logo, -ERANGE for a short buffer,
/// -EPERM for any caller but the splash.
#[no_mangle]
pub extern "C" fn mk_boot_logo(which: u32, out: *mut u8, len: usize) -> i64 {
    call_raw(N_MK_BOOT_LOGO, [which as u64, out as u64, len as u64, 0, 0, 0])
}
