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

//! A frame that outgrew its surface's pixels, said once a run: it comes
//! again each frame until the new surface takes over, and a line a frame
//! filled the Terminal.

use core::sync::atomic::{AtomicBool, Ordering};

static SAID: AtomicBool = AtomicBool::new(false);
static NO_HEAP: AtomicBool = AtomicBool::new(false);

pub(super) fn outgrown() {
    if !SAID.swap(true, Ordering::Relaxed) {
        let line = b"[WAYLAND] a frame outgrew its surface; a new one is made\n";
        let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    }
}

/// The heap had no room for a window's pixels: the frame is not shown, and
/// the window is tried again at its next frame.
pub(in crate::linux::wayland::present) fn no_heap(bytes: usize) {
    if !NO_HEAP.swap(true, Ordering::Relaxed) {
        let line = alloc::format!("[WAYLAND] no room for a {bytes} byte frame, not shown\n");
        let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    }
}
