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

use super::program_max::PROGRAM_MAX;

/// An install holds a distribution's index while it resolves a closure.
/// Kali's main is 21 MB fetched and 85 MB inflated, parsed into records
/// beside it; Alpine's is a few.
const INSTALL_HEAP: usize = 320 << 20;

/// A run holds the program it loads, read whole from the store into one
/// allocation of its size (file/store/bytes.rs), beside the family's own
/// state. 64 MiB holds a program up to about 48 MiB.
const RUN_HEAP: usize = 64 << 20;

/// With memory to spare, a run takes room for a program up to PROGRAM_MAX
/// (simplex-chat is 79.5 MB) and 32 MiB beside it. The heap is backed when
/// it is made, so a machine with less than 2 GiB free keeps RUN_HEAP.
const LARGE_HEAP: usize = PROGRAM_MAX as usize + (32 << 20);
const SPARE_KB: u64 = 2 << 20;

pub fn init() {
    let mut buf = [0u8; nonos_libc::ARGV_BYTES_MAX];
    let n = mk_args(buf.as_mut_ptr(), buf.len());
    if super::heap_choice::first_word(&buf, n).is_none() && n > 0 {
        let line = b"[LINUX] arguments longer than 4 KiB: taking the run heap\n";
        let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    }
    if super::heap_choice::is_install(&buf, n) {
        // Twice that with memory to spare: one package is placed at about
        // twice its size (install/fetch/room.rs), Telegram's past 100 MB.
        let sizes = [INSTALL_HEAP * 2, INSTALL_HEAP, INSTALL_HEAP / 2];
        if sizes.iter().skip(usize::from(!spare())).any(|&s| heap_init_sized(s).is_ok()) {
            return;
        }
        // Alpine's index still fits the default; a larger one fails where
        // it is read, with that reason, instead of here without one.
        let line = b"[LINUX] no room for a large index, installing in the default heap\n";
        let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    } else if spare() && heap_init_sized(LARGE_HEAP).is_ok() {
        return;
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

/// Whether the machine has SPARE_KB free for the large heap.
fn spare() -> bool {
    super::file::free_kb().is_some_and(|kb| kb >= SPARE_KB)
}
