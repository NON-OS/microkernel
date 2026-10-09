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

mod blk;
mod protocol;
mod server;
mod store;

use nonos_libc::{heap_init_sized, mk_exit};

/// The whole store is loaded into this heap, up to MAX_TOTAL_BYTES (96 MiB),
/// with room beside it for a replaced file's new bytes and the buffers the
/// server works in: 3.2 times the store, rounded up. The kernel backs a page
/// only when it is first touched, so the size reserves address space, not
/// RAM a small store never uses.
const VFS_HEAP: usize = 320 * 1024 * 1024;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init_sized(VFS_HEAP).is_err() {
        mk_exit(1);
    }
    server::run();
}
