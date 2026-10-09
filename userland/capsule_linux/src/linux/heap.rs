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
/// beside it; Alpine's is a few.
const INSTALL_HEAP: usize = 320 << 20;

/// A run holds the program it loads, read whole from the store, beside the
/// family's own state. A 6 MB Go program outgrew the 16 MiB default while it
/// was read.
///
/// This is not the largest program that may run. The read buffer doubles as it
/// fills, so a program of N bytes costs N/2 + N at the peak: 64 MiB covers a
/// program up to about 32 MiB, not up to 64. Two ceilings sit above that and
/// they are different numbers: a program carrying a certificate is verified by
/// the kernel up to 16 MiB (`capsule_load/copy.rs`), one without a certificate
/// locally up to 64 MiB (`local_image.rs`). A program between 32 and 64 MiB
/// therefore passes verification and still ends the personality here, which
/// wants a buffer sized from the file rather than a larger constant.
const RUN_HEAP: usize = 64 << 20;

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
    } else if heap_init_sized(RUN_HEAP).is_ok() {
        return;
    }
    if heap_init_sized(RUN_HEAP).is_ok() {
        return;
    }
    let line = b"[LINUX] no room for a 40 MiB heap, running in the default 16 MiB\n";
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    let _ = heap_init();
}
