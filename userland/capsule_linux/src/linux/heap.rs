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

//! How much memory this process takes, decided from its arguments before
//! anything is allocated.

use nonos_libc::{heap_init, heap_init_sized, mk_args};

/// An install holds a distribution's index while it resolves a closure.
/// Kali's main is 21 MB fetched and 85 MB inflated, parsed into records
/// beside it; Alpine's is a few. A run takes the default.
const INSTALL_HEAP: usize = 320 << 20;

pub fn init() {
    let mut buf = [0u8; 256];
    let n = mk_args(buf.as_mut_ptr(), buf.len());
    if n > 0 && buf.starts_with(b"install\0") {
        if heap_init_sized(INSTALL_HEAP).is_ok() {
            return;
        }
        // Alpine's index still fits the default; a larger one fails where
        // it is read, with that reason, instead of here without one.
        let line = b"[LINUX] no room for a large index, installing in the default heap\n";
        let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    }
    let _ = heap_init();
}
