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

//! `MkBootLogo(which, out, len) -> bytes`: the logo firmware drew (BGRT),
//! read-only, for the boot splash to keep it up without a flash. Which 0:
//! a 36-byte record, the logo's x, y, width and height u32, flags u8 (bit
//! 0 on screen now, bit 1 rows top-down), the clockwise quarter turns u8,
//! bits a pixel u16, the BMP file's size u32 and its pixels' offset u32,
//! then the width and height u32 of the firmware mode x and y are offsets
//! into (0 when there was no framebuffer). The first 28 bytes are as they
//! were, so a reader that takes 28 still works.
//! Which 1: the BMP file itself, copied at boot and checked before it was
//! kept. A null `out` asks the length. Only the boot splash's owner may
//! call it. ENOENT without a logo; ERANGE for a short buffer.

use crate::hardware::broker::acpi::boot_logo::boot_logo_file;
use crate::hardware::broker::acpi::logo_record::{logo_record, record_len};
use crate::process::current_pid;
use crate::services::registry::lookup_service;
use crate::syscall::microkernel::errnos::{
    ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOENT, ERRNO_PERM, ERRNO_RANGE,
};
use crate::usercopy::copy_to_user;

const SPLASH: &str = "app.boot_splash";

fn give(bytes: &[u8], out_ptr: u64, out_len: u64) -> i64 {
    let len = bytes.len() as u64;
    match (out_ptr, out_len >= len) {
        (0, _) => len as i64,
        (_, false) => ERRNO_RANGE,
        _ => copy_to_user(out_ptr, bytes).map_or(ERRNO_FAULT, |()| len as i64),
    }
}

pub fn sys_boot_logo(which: u64, out_ptr: u64, out_len: u64) -> i64 {
    let owner = lookup_service(SPLASH).map(|e| e.pid);
    if owner.is_none() || owner != current_pid() {
        return ERRNO_PERM;
    }
    match which {
        0 => {
            logo_record().map_or(ERRNO_NOENT, |r| give(&r[..record_len(out_len)], out_ptr, out_len))
        }
        1 => boot_logo_file(|f| give(f, out_ptr, out_len)).unwrap_or(ERRNO_NOENT),
        _ => ERRNO_INVAL,
    }
}
