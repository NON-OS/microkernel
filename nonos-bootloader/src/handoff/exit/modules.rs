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

//! The module array the handoff points at: the loader image and the kernel
//! image file, for the installer. Written into the loader-data page
//! allocated for it, so it outlives boot services like the handoff itself.

use super::params::HandoffInitParams;
use crate::handoff::types::{BootHandoffV1, Module};

/// A region the loader could not record has size zero and is skipped, so
/// the kernel sees only modules that exist; the installer then reports the
/// missing one rather than writing a disk with no bootloader on it.
///
/// # Safety
/// `bh_ptr` and `p.modules_addr` point at loader-data allocations of at
/// least one handoff struct and one page respectively.
pub unsafe fn init_modules(bh_ptr: *mut BootHandoffV1, p: &HandoffInitParams) {
    let slots = p.modules_addr as *mut Module;
    let mut count = 0u32;
    for m in p.install_source.iter().filter(|m| m.size > 0) {
        core::ptr::write(slots.add(count as usize), *m);
        count += 1;
    }
    (*bh_ptr).modules.ptr = if count > 0 { p.modules_addr } else { 0 };
    (*bh_ptr).modules.count = count;
    (*bh_ptr).modules.reserved = 0;
}
