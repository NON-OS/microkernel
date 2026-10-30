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

//! The line for a GOP backing the kernel would not register: the
//! bytes it asked for, and the ceiling when it is past it or the errno when
//! not, said once though the setup asks again every 20 ms, so a panel past
//! the kernel's ceiling says why the screen stays black. Pure:
//! compositor_proofs::tests::rejected.

extern crate alloc;

use alloc::{format, string::String};
use core::sync::atomic::{AtomicBool, Ordering};

/// The kernel's ceiling on one surface (surface_handlers/cap.rs CEILING),
/// mirrored; kernel_proofs::surface_cap holds the two equal.
pub const SURFACE_CEILING: u64 = 128 * 1024 * 1024;

/// Whether the line was said this run.
pub static SAID: AtomicBool = AtomicBool::new(false);

/// The line for a backing of `byte_len` bytes the kernel refused with `rc`,
/// the negative errno the register call returned.
pub fn line(byte_len: u64, rc: i64) -> String {
    let asked = format!("gop: surface register rejected: {byte_len} bytes asked");
    if byte_len > SURFACE_CEILING {
        format!("{asked}; past the kernel's ceiling of 128 MiB")
    } else {
        format!("{asked}; refused, errno {}", -rc)
    }
}

/// Say the line through `say` the first time only.
pub fn say_once(byte_len: u64, rc: i64, said: &AtomicBool, say: impl FnOnce(&str)) {
    if !said.swap(true, Ordering::Relaxed) {
        say(&line(byte_len, rc));
    }
}
