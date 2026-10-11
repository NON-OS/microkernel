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

//! A window of the screen, and a ceiling the kernel and heap both hold.

use crate::frame_len::{frame_len, MAX_FRAME};

/// The kernel registers a surface of at most this many bytes
/// (src/syscall/dispatch/router/surface_handlers.rs MAX_SURFACE_BYTES).
const KERNEL_SURFACE: u64 = 64 << 20;

#[test]
fn a_window_of_the_screen_is_shown() {
    let (w, h) = (1920u32, 1080u32);
    assert_eq!(frame_len(0, w * 4, h, u64::from(w * 4 * h)), Some((w * 4 * h) as usize));
    assert_eq!(frame_len(4096, 2560 * 4, 1600, 64 << 20), Some(2560 * 4 * 1600));
}

#[test]
fn the_ceiling_fits_the_kernel_and_the_heap() {
    const { assert!(MAX_FRAME <= KERNEL_SURFACE / 4, "a quarter of what the kernel registers") };
    const { assert!(MAX_FRAME >= 1920 * 1080 * 4, "the whole screen fits") };
}
