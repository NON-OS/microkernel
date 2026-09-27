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

//! The one thing align_up promises: its result is never below its input.
//!
//! Two implementations used to break it at the top of the address space, by
//! different routes. boot_memory reached saturating_add, which pinned the sum at
//! the maximum and let the mask clear the low bits. buddy_alloc caught the
//! overflow with checked_add and then returned `MAX & !(align - 1)` in the arm
//! that handled it. Both returned an address strictly below the value they were
//! asked to round up, which is the direction that turns a bounds check into a
//! pass.
//!
//! These run against the kernel source through `#[path]`, so they fail if either
//! regresses.

use crate::memory::align::{boot_align_down, boot_align_up, buddy_align_up};

/// Deterministic spread, so a failure is reproducible from the seed alone.
fn xorshift(state: &mut u64) -> u64 {
    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *state = x;
    x
}

/// A value biased toward the boundary that matters.
///
/// Uniform 64 bit values are useless for this property. Overflow needs the value
/// within one alignment of the top, so with alignments up to 2^31 a uniform draw
/// finds it with probability around 2^-33 and two hundred thousand of them find
/// it never. An earlier version of these tests sampled uniformly, passed, and
/// did not notice when the fix was reverted. A quarter of the draws now sit
/// within a small window of the maximum.
fn biased(state: &mut u64) -> u64 {
    let r = xorshift(state);
    match r % 4 {
        0 => u64::MAX - (xorshift(state) % 65_536),
        1 => u64::MAX - (xorshift(state) % 32),
        _ => r,
    }
}

#[test]
fn boot_align_up_is_never_below_its_input() {
    let mut s = 0x9e37_79b9_7f4a_7c15u64;
    for _ in 0..200_000 {
        let value = biased(&mut s);
        let align = 1u64 << (xorshift(&mut s) % 32);
        let out = boot_align_up(value, align);
        assert!(out >= value, "align_up({value:#x}, {align:#x}) = {out:#x}, below its input");
    }
}

#[test]
fn buddy_align_up_is_never_below_its_input() {
    let mut s = 0x2545_f491_4f6c_dd1du64;
    for _ in 0..200_000 {
        let value = biased(&mut s) as usize;
        let align = 1usize << (xorshift(&mut s) % 32);
        let out = buddy_align_up(value, align);
        assert!(out >= value, "align_up({value:#x}, {align:#x}) = {out:#x}, below its input");
    }
}

#[test]
fn boot_align_down_is_never_above_its_input() {
    let mut s = 0x1234_5678_9abc_def0u64;
    for _ in 0..200_000 {
        let value = biased(&mut s);
        let align = 1u64 << (xorshift(&mut s) % 32);
        let out = boot_align_down(value, align);
        assert!(out <= value, "align_down({value:#x}, {align:#x}) = {out:#x}, above its input");
    }
}

/// The exact witness that used to fail, kept so the regression has a name.
/// Before the fix this returned 0xFFFF_FFFF_FFFF_F000.
#[test]
fn the_top_of_the_address_space_does_not_wrap_downward() {
    let top = u64::MAX;
    assert_eq!(boot_align_up(top, 4096), top);
    assert_eq!(buddy_align_up(usize::MAX, 4096), usize::MAX);
}

/// One page below the top still has room to round up, so the boundary is where
/// it should be and not one page early.
#[test]
fn one_page_below_the_top_still_rounds_up() {
    let value = u64::MAX - 4095;
    assert_eq!(boot_align_up(value, 4096), value);
    assert_eq!(boot_align_up(u64::MAX - 4096, 4096), u64::MAX - 4095);
}

/// The ordinary case is unchanged by the fix.
#[test]
fn the_ordinary_case_still_rounds() {
    assert_eq!(boot_align_up(4097, 4096), 8192);
    assert_eq!(boot_align_up(4096, 4096), 4096);
    assert_eq!(boot_align_down(4097, 4096), 4096);
    assert_eq!(buddy_align_up(4097, 4096), 8192);
}

/// An alignment of zero or one that is not a power of two is declined rather
/// than applied. This is existing behaviour and is pinned so a later change to
/// these helpers has to be deliberate about it.
#[test]
fn a_zero_or_non_power_of_two_alignment_is_declined() {
    assert_eq!(boot_align_up(10, 0), 10);
    assert_eq!(boot_align_up(10, 3), 10);
    assert_eq!(boot_align_down(10, 0), 10);
    assert_eq!(buddy_align_up(10, 3), 10);
}
