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

//! The partition this boot came from, as the loader recorded it, read once,
//! whether the loader came through USB, and whether it read its copy of the
//! store from that same disk.

use spin::Once;

use super::gpt::boot_guid;
use crate::boot::handoff::types::{BOOT_MEDIA_LEN, BOOT_MEDIA_USB, MODULE_KIND_BOOT_MEDIA};
use crate::boot::handoff::types::{MODULE_KIND_STORE, STORE_FROM_BOOT_DISK};
use crate::syscall::microkernel::{install_source_modules, install_source_window};

static BOOT: Once<Option<[u8; BOOT_MEDIA_LEN]>> = Once::new();

/// The GPT partition GUID the loader was read from, `None` when it named
/// none (an MBR stick, a firmware FAT drive) or left no record.
pub(super) fn partition() -> Option<[u8; 16]> {
    BOOT.call_once(read).as_ref().and_then(|r| boot_guid(r))
}

/// True when the loader's volume path has a USB node: a stick boot.
pub(super) fn from_usb() -> bool {
    BOOT.call_once(read).is_some_and(|r| u16::from_le_bytes([r[6], r[7]]) & BOOT_MEDIA_USB != 0)
}

/// True when the loader marked its store copy as read from the disk it came
/// from, and the copy is one the kernel answers store reads from. A loader
/// that leaves the mark off, or read another disk's store, gives false.
pub(super) fn store_from_boot_disk() -> bool {
    let marked = install_source_modules::find(MODULE_KIND_STORE as u64)
        .is_some_and(|m| m.reserved & STORE_FROM_BOOT_DISK != 0);
    marked && super::store_copy::present()
}

fn read() -> Option<[u8; BOOT_MEDIA_LEN]> {
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
    record.try_into().ok()
}
