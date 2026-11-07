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

//! A freed capsule allocation is unreadable afterwards.
//!
//! Runs the real ZeroOnFree over a region this test owns so the block can be
//! read back after dealloc. Without the wipe, 48 of 64 secret bytes survived.
//! Volatile both ways: the compiler may assume nothing reads a freed block, so
//! a plain read folds against the writes and passes regardless.

use core::alloc::{GlobalAlloc, Layout};

use crate::heap::zero_on_free::ZeroOnFree;

const SECRET: u8 = 0xAB;

/// Allocate, fill with `SECRET`, free, and report how many secret bytes are
/// still there. The region leaks deliberately: the allocator holds it for the
/// rest of the process.
fn surviving_bytes_after_free(size: usize) -> usize {
    const REGION: usize = 1 << 16;
    let backing: *mut u8 = Box::leak(vec![0u8; REGION].into_boxed_slice()).as_mut_ptr();
    let allocator = ZeroOnFree::empty();
    // SAFETY: the region is freshly allocated, unaliased, and leaked, so it
    // outlives the allocator that is about to own it.
    unsafe { allocator.init(backing, REGION) };

    let layout = Layout::from_size_align(size, 8).expect("valid layout");
    // SAFETY: the allocator was just initialised over a region far larger than
    // this request.
    let block = unsafe { allocator.alloc(layout) };
    assert!(!block.is_null(), "allocation failed");

    for i in 0..size {
        // SAFETY: `i < size` and the block is `size` bytes this test owns.
        unsafe { core::ptr::write_volatile(block.add(i), SECRET) };
    }
    // SAFETY: the block came from this allocator with this layout and is not
    // used as a live allocation again; the reads below are of freed memory the
    // test still owns through `backing`.
    unsafe { allocator.dealloc(block, layout) };

    (0..size)
        .filter(|&i| {
            // SAFETY: within the leaked region, which is still mapped.
            unsafe { core::ptr::read_volatile(block.add(i)) == SECRET }
        })
        .count()
}

#[test]
fn a_freed_capsule_allocation_keeps_no_secret() {
    for size in [16usize, 64, 256, 4096] {
        assert_eq!(
            surviving_bytes_after_free(size),
            0,
            "a {size} byte block still held its contents after being freed"
        );
    }
}

#[test]
fn the_wipe_covers_sizes_that_are_not_multiples_of_a_word() {
    // The wipe goes a word at a time between its first and last boundary,
    // so an odd tail must be covered too: one that rounded down would leave
    // the last bytes readable.
    for size in [1usize, 7, 9, 63, 65, 4095, 40_000 + 3] {
        assert_eq!(surviving_bytes_after_free(size), 0, "a {size} byte block kept its tail");
    }
}

/// A heap grown at its end (libc heap/grow.rs, vfs's budget: track 8b):
/// the first region full, a request that did not fit before fits once the
/// next piece is handed over, and lands inside it.
#[test]
fn an_extended_heap_serves_from_its_new_end() {
    const HALF: usize = 1 << 15;
    let backing: *mut u8 = Box::leak(vec![0u8; 2 * HALF].into_boxed_slice()).as_mut_ptr();
    let allocator = ZeroOnFree::empty();
    // SAFETY: the first half is the region; the whole buffer is leaked, so
    // the second half handed over below outlives the allocator too.
    unsafe { allocator.init(backing, HALF) };
    let big = Layout::from_size_align(HALF + HALF / 2, 8).expect("valid layout");
    // SAFETY: a valid layout; null means it did not fit.
    assert!(unsafe { allocator.alloc(big) }.is_null(), "more than the region holds");
    // SAFETY: the second half begins where the region ends and is this
    // test's, unaliased, for the rest of the run.
    unsafe { allocator.extend(HALF) };
    // SAFETY: as above.
    let block = unsafe { allocator.alloc(big) };
    assert!(!block.is_null(), "fits once grown");
    let end = backing as usize + 2 * HALF;
    assert!(block as usize >= backing as usize && block as usize + big.size() <= end);
}

/// libc maps a heap in 64 MiB calls (heap/span.rs): MkMmap holds its CPU to
/// the last page, and only a tick from user mode switches, so one call of a
/// gigabyte held a one-CPU guest for seconds (track 8b). Growth goes the
/// same way, a piece at most.
#[test]
fn the_heap_is_mapped_in_pieces_no_call_longer_than_64_mib() {
    let span = include_str!("../../libc/src/heap/span.rs");
    assert!(span.contains("pub(super) const PIECE: usize = 64 << 20;"));
    assert!(span.contains("let len = (bytes - done).min(PIECE);"), "every later piece too");
    let grow = include_str!("../../libc/src/heap/grow.rs");
    assert!(grow.contains("if bytes == 0 || bytes > super::span::PIECE {"));
}

/// Each copy's wipe over `len` bytes at `off` into a buffer of `SECRET`
/// with guards either side: how many bytes inside kept the secret, and how
/// many outside it lost.
fn wipe_at(wipe: unsafe fn(*mut u8, usize), off: usize, len: usize) -> (usize, usize) {
    const GUARD: usize = 24;
    let mut buf = vec![SECRET; GUARD + off + len + GUARD];
    // SAFETY: `GUARD + off + len` bytes from the start lie inside `buf`.
    unsafe { wipe(buf.as_mut_ptr().add(GUARD + off), len) };
    let inside = GUARD + off..GUARD + off + len;
    let kept = buf[inside.clone()].iter().filter(|&&b| b == SECRET).count();
    let lost = buf.iter().enumerate().filter(|(i, &b)| !inside.contains(i) && b != SECRET).count();
    (kept, lost)
}

#[test]
fn the_wipe_clears_every_byte_and_no_byte_beside_it_at_every_alignment() {
    use crate::heap::zero_on_free::wipe as libc_wipe;
    use crate::heap::zero_on_free_alloc::wipe as alloc_wipe;
    for wipe in [libc_wipe as unsafe fn(*mut u8, usize), alloc_wipe] {
        for off in 0..16 {
            for len in [0usize, 1, 2, 7, 8, 9, 15, 16, 17, 31, 63, 64, 65, 4095, (1 << 20) + 5] {
                assert_eq!(wipe_at(wipe, off, len), (0, 0), "at {off}, {len} bytes");
            }
        }
    }
}

#[test]
fn a_freed_frame_sized_block_keeps_no_byte() {
    // A 1280x720 window's pixels and its page of slack, as the Wayland
    // bridge frees one on every resize step.
    const FRAME: usize = 1280 * 720 * 4 + 4096;
    let region = FRAME + (1 << 16);
    let backing: *mut u8 = Box::leak(vec![0u8; region].into_boxed_slice()).as_mut_ptr();
    let allocator = ZeroOnFree::empty();
    // SAFETY: freshly allocated, unaliased and leaked, so it outlives the
    // allocator that owns it now.
    unsafe { allocator.init(backing, region) };
    let layout = Layout::from_size_align(FRAME, 1).expect("valid layout");
    // SAFETY: the region is larger than the request.
    let block = unsafe { allocator.alloc(layout) };
    assert!(!block.is_null());
    // SAFETY: the block is FRAME bytes this test owns until the dealloc.
    unsafe { core::ptr::write_bytes(block, SECRET, FRAME) };
    // SAFETY: from this allocator with this layout, not used again.
    unsafe { allocator.dealloc(block, layout) };
    let kept = (0..FRAME)
        .filter(|&i| {
            // SAFETY: within the leaked region, still mapped.
            unsafe { core::ptr::read_volatile(block.add(i)) == SECRET }
        })
        .count();
    assert_eq!(kept, 0);
}

/// The time to wipe a 1280x720 frame's pixels, a byte at a time as before
/// and as the allocators do now:
/// cargo test --release wipe_numbers -- --ignored --nocapture
#[test]
#[ignore]
fn wipe_numbers() {
    use std::time::Instant;
    const FRAME: usize = 1280 * 720 * 4;
    let mut buf = vec![SECRET; FRAME];
    let by_byte = |p: *mut u8, n: usize| {
        for i in 0..n {
            // SAFETY: `i < n`, inside `buf`.
            unsafe { core::ptr::write_volatile(p.add(i), 0) };
        }
    };
    let time = |f: &dyn Fn(*mut u8, usize), p: *mut u8| {
        let mut t: Vec<f64> = (0..64)
            .map(|_| {
                let at = Instant::now();
                f(p, FRAME);
                at.elapsed().as_secs_f64() * 1e6
            })
            .collect();
        t.sort_by(f64::total_cmp);
        (t[32], t[63])
    };
    let p = buf.as_mut_ptr();
    let before = time(&by_byte, p);
    // SAFETY: `buf` is FRAME bytes.
    let after = time(&|p, n| unsafe { crate::heap::zero_on_free::wipe(p, n) }, p);
    println!(
        "wipe 1280x720 frame: by byte p50 {:.0} us p99 {:.0} us; by word p50 {:.0} us p99 {:.0} us",
        before.0, before.1, after.0, after.1
    );
}
