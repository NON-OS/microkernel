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

//! `MkInstallSource`: the bytes of the image this machine is running.
//!
//! The bootloader loads `kernel.bin` into memory to verify it and leaves it
//! there; its own image sits where the firmware placed it. Both are in
//! loader memory, which the kernel never hands to its allocator, so they are
//! intact for the life of the boot. The installer asks for them by kind and
//! offset and writes them to a disk, and what lands on that disk is the
//! exact image that was verified and booted, not a copy read back off the
//! medium it came from. That is what lets an install need no source disk
//! and no USB storage driver.

use super::errnos::{ERRNO_FAULT, ERRNO_NOENT};
use super::{install_source_modules, install_source_window};

/// Module kinds the bootloader records, shared with its `Module::kind`.
pub const KIND_LOADER_IMAGE: u32 = 1;
pub const KIND_KERNEL_IMAGE: u32 = 2;

/// `kind`, `offset` into that image, destination and its length. Returns the
/// bytes written, zero at the end of the image, so a caller learns the size
/// by reading to the end and never needs a separate length call.
pub fn sys_install_source(kind: u64, offset: u64, out_ptr: u64, out_len: u64) -> i64 {
    let Some(m) = install_source_modules::find(kind) else {
        return ERRNO_NOENT;
    };
    let (virt, want) = match install_source_window::window(m.base, m.size, offset, out_len) {
        Ok(w) => w,
        Err(e) => return e,
    };
    if want == 0 {
        return 0;
    }
    // SAFETY: eK@nonos.systems - `base..base+size` was recorded by the loader
    // from an allocation it owned, `offset + want <= size` is checked above,
    // and the directmap covers every physical address the loader can hand us.
    let src = unsafe { core::slice::from_raw_parts(virt.as_u64() as *const u8, want as usize) };
    if crate::usercopy::write_user_bytes(out_ptr, src).is_err() {
        return ERRNO_FAULT;
    }
    want as i64
}
