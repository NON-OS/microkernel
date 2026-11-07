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


//! The user-files broker capsule.

#![no_std]
#![no_main]

extern crate alloc;

mod desk;
mod runner;
mod volume;

use nonos_libc::{heap_init_sized, mk_exit};

/// The broker's heap: changed chunks (16 MiB at most), the manifest of the
/// largest tree it keeps, and two messages. The kernel backs a page only
/// when it is first touched.
const HEAP: usize = 96 << 20;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init_sized(HEAP).is_err() {
        mk_exit(1);
    }
    runner::run();
}
