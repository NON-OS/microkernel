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

//! The line for a GOP backing the kernel would not register (H-06): the
//! bytes it asked for and the most the kernel maps, said once though the
//! setup asks again every 20 ms, so a panel past the kernel's ceiling says
//! why the screen stays black. Pure: compositor_proofs::tests::rejected.

extern crate alloc;

use alloc::{format, string::String};
use core::sync::atomic::{AtomicBool, Ordering};

/// The kernel's ceiling on one surface (surface_handlers/cap.rs CEILING),
/// mirrored; kernel_proofs::surface_cap holds the two equal.
pub const SURFACE_CEILING: u64 = 128 * 1024 * 1024;

/// Whether the line was said this run.
pub static SAID: AtomicBool = AtomicBool::new(false);

/// The line for a backing of `byte_len` bytes the kernel refused.
pub fn line(byte_len: u64) -> String {
    format!(
        "gop: surface register rejected: {} bytes asked; the kernel maps at most {} \
         (128 MiB), and less unless the boot framebuffer is that large",
        byte_len, SURFACE_CEILING
    )
}

/// Say the line through `say` the first time only.
pub fn say_once(byte_len: u64, said: &AtomicBool, say: impl FnOnce(&str)) {
    if !said.swap(true, Ordering::Relaxed) {
        say(&line(byte_len));
    }
}
