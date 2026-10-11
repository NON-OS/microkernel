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

//! The partition this boot came from, as the loader recorded it, read once.

use spin::Once;

use super::gpt::boot_guid;
use crate::boot::handoff::types::{BOOT_MEDIA_LEN, MODULE_KIND_BOOT_MEDIA};
use crate::syscall::microkernel::{install_source_modules, install_source_window};

static BOOT: Once<Option<[u8; 16]>> = Once::new();

/// The GPT partition GUID the loader was read from, `None` when it named
/// none (an MBR stick, a firmware FAT drive) or left no record.
pub(super) fn partition() -> Option<[u8; 16]> {
    *BOOT.call_once(read)
}

fn read() -> Option<[u8; 16]> {
    let m = install_source_modules::find(MODULE_KIND_BOOT_MEDIA as u64)?;
    let len = BOOT_MEDIA_LEN as u64;
    let (virt, n) = install_source_window::window(m.base, m.size, 0, len).ok()?;
    if n != len {
        return None;
    }
    // SAFETY: the loader recorded `base..base+size` in loader memory, which
    // the kernel never frees; `window` checked both ends against the
    // directmap and `n` bytes lie inside the record.
    let record = unsafe { core::slice::from_raw_parts(virt.as_u64() as *const u8, n as usize) };
    boot_guid(record)
}
