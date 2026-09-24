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

#![no_std]
#![no_main]

extern crate alloc;

mod install;

use nonos_app_skeleton::run;
use nonos_libc::{
    heap_init_sized, mk_install_source_size, INSTALL_SOURCE_KERNEL_IMAGE,
    INSTALL_SOURCE_LOADER_IMAGE,
};

/// Room beyond the image copy for the window, the fonts, the disk writer's
/// chunks and the read-back buffers.
const HEADROOM: usize = 32 * 1024 * 1024;
const MIB: usize = 1024 * 1024;

/// The heap this capsule needs: the running image is copied into it once,
/// and the default sixteen mebibytes hold a sixth of a kernel. Sized from
/// the kernel's answer rather than a constant, so a larger kernel does not
/// turn into an allocation failure with no window to report it in. With no
/// image recorded the skeleton's default heap is enough to say so.
fn heap_bytes() -> Option<usize> {
    let loader = mk_install_source_size(INSTALL_SOURCE_LOADER_IMAGE)? as usize;
    let kernel = mk_install_source_size(INSTALL_SOURCE_KERNEL_IMAGE)? as usize;
    Some((loader + kernel + HEADROOM).div_ceil(MIB) * MIB)
}

/// # Safety
/// The kernel enters here once, on a fresh stack, before anything else in
/// this capsule has run.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if let Some(bytes) = heap_bytes() {
        let _ = heap_init_sized(bytes);
    }
    run(install::Install::new)
}
