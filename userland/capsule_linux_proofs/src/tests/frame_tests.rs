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

//! The frame a Wayland commit copies into this capsule's heap. A pool is
//! as large as the client's mapping, which a reservation makes as large as
//! it likes for nothing, so the frame is bounded by MAX_FRAME and never by
//! what the client claims.

use super::random::Regs;
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

/// The guest that ended the family: a memfd mapped PROT_NONE over two
/// gigabytes, a pool over it, and a 16384 by 16384 buffer committed.
#[test]
fn a_buffer_larger_than_a_frame_is_refused_whatever_the_pool() {
    let pool = 2u64 << 30;
    assert_eq!(frame_len(0, 16384 * 4, 16384, pool), None);
    assert_eq!(frame_len(0, u32::MAX, u32::MAX, u64::MAX), None);
    assert_eq!(frame_len(0, 1, (MAX_FRAME + 1) as u32, u64::MAX), None, "one byte over");
    assert_eq!(frame_len(0, 1, MAX_FRAME as u32, u64::MAX), Some(MAX_FRAME as usize));
}

#[test]
fn a_buffer_outside_its_pool_or_empty_is_refused() {
    assert_eq!(frame_len(1, 4096, 1, 4096), None, "one byte past the pool");
    assert_eq!(frame_len(u64::MAX, 4, 1, u64::MAX), None, "an offset that wraps");
    assert_eq!(frame_len(0, 0, 100, 1 << 20), None);
    assert_eq!(frame_len(0, 100, 0, 1 << 20), None);
}

#[test]
fn the_ceiling_fits_the_kernel_and_the_heap() {
    const { assert!(MAX_FRAME <= KERNEL_SURFACE / 4, "a quarter of what the kernel registers") };
    const { assert!(MAX_FRAME >= 1920 * 1080 * 4, "the whole screen fits") };
}

/// Over any buffer and pool: a frame shown is inside its pool, not empty,
/// and never more than MAX_FRAME.
#[test]
fn no_buffer_makes_a_frame_past_the_ceiling() {
    let mut r = Regs::new(0x00F2_A3E5);
    for _ in 0..200_000 {
        let (offset, pool) = (r.arg(), r.arg());
        let (stride, height) = (r.arg() as u32, r.arg() as u32);
        if let Some(bytes) = frame_len(offset, stride, height, pool) {
            let bytes = bytes as u64;
            assert!(bytes > 0 && bytes <= MAX_FRAME, "{stride}x{height}: {bytes}");
            assert_eq!(bytes, u64::from(stride) * u64::from(height));
            assert!(offset + bytes <= pool, "{offset:#x}+{bytes} in {pool:#x}");
        }
    }
}
