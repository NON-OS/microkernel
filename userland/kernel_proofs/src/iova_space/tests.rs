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

use super::space::{
    after_give_back, place, touches_interrupt_window, INTERRUPT_WINDOW_END,
    INTERRUPT_WINDOW_START, IOVA_BASE, IOVA_LIMIT,
};

const PAGE: u64 = 4096;
const MIB: u64 = 1 << 20;

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

#[test]
fn the_first_grant_starts_at_the_base() {
    assert_eq!(place(IOVA_BASE, PAGE), Some((IOVA_BASE, IOVA_BASE + PAGE)));
    // A pointer below the base, which nothing should ever hold, is lifted.
    assert_eq!(place(0, PAGE), Some((IOVA_BASE, IOVA_BASE + PAGE)));
}

#[test]
fn a_grant_that_would_touch_the_interrupt_window_starts_past_it() {
    // Ends exactly at the window: allowed, it does not touch it.
    let next = INTERRUPT_WINDOW_START - 2 * PAGE;
    assert_eq!(place(next, 2 * PAGE), Some((next, INTERRUPT_WINDOW_START)));
    // One page more would cross into it.
    let (start, end) = place(next, 3 * PAGE).expect("fits past the window");
    assert_eq!(start, INTERRUPT_WINDOW_END);
    assert_eq!(end, INTERRUPT_WINDOW_END + 3 * PAGE);
    // A pointer already inside the window jumps out of it.
    let inside = INTERRUPT_WINDOW_START + 5 * PAGE;
    assert_eq!(place(inside, PAGE).map(|p| p.0), Some(INTERRUPT_WINDOW_END));
}

#[test]
fn no_placement_ever_overlaps_the_interrupt_window() {
    let mut s = 0x9E37_79B9_7F4A_7C15u64;
    for _ in 0..200_000 {
        let next = IOVA_BASE + (xorshift(&mut s) % (IOVA_LIMIT - IOVA_BASE)) / PAGE * PAGE;
        let length = (1 + xorshift(&mut s) % (64 * MIB / PAGE)) * PAGE;
        if let Some((start, end)) = place(next, length) {
            assert!(!touches_interrupt_window(start, length), "{start:#x}+{length:#x}");
            assert_eq!(end, start + length);
            assert!(start >= next.max(IOVA_BASE));
            assert!(end <= IOVA_LIMIT);
        }
    }
}

#[test]
fn a_bump_allocator_run_to_exhaustion_never_hands_out_the_window() {
    let mut next = IOVA_BASE;
    let mut count = 0u64;
    let length = 8 * MIB;
    while let Some((start, end)) = place(next, length) {
        assert!(!touches_interrupt_window(start, length));
        assert!(start >= next);
        next = end;
        count += 1;
    }
    // Everything below 4 GiB but the window and the first megabyte was used.
    assert!(next <= IOVA_LIMIT);
    assert!(count >= (IOVA_LIMIT - IOVA_BASE - (INTERRUPT_WINDOW_END - INTERRUPT_WINDOW_START)) / length - 1);
}

#[test]
fn nothing_is_placed_past_the_limit_or_for_nothing() {
    assert_eq!(place(IOVA_LIMIT - PAGE, 2 * PAGE), None);
    assert_eq!(place(IOVA_LIMIT, PAGE), None);
    assert_eq!(place(u64::MAX - PAGE, 2 * PAGE), None);
    assert_eq!(place(IOVA_BASE, 0), None);
    assert_eq!(place(IOVA_BASE, u64::MAX), None);
    assert_eq!(place(IOVA_LIMIT - PAGE, PAGE), Some((IOVA_LIMIT - PAGE, IOVA_LIMIT)));
}

#[test]
fn the_window_test_is_exact_at_both_edges() {
    assert!(!touches_interrupt_window(INTERRUPT_WINDOW_START - PAGE, PAGE));
    assert!(touches_interrupt_window(INTERRUPT_WINDOW_START - PAGE, PAGE + 1));
    assert!(touches_interrupt_window(INTERRUPT_WINDOW_START, 1));
    assert!(touches_interrupt_window(INTERRUPT_WINDOW_END - 1, 1));
    assert!(!touches_interrupt_window(INTERRUPT_WINDOW_END, PAGE));
    // A run that wraps the address space is treated as touching everything.
    assert!(touches_interrupt_window(u64::MAX, 2));
    // The window is the architectural one.
    assert_eq!(INTERRUPT_WINDOW_START, 0xFEE0_0000);
    assert_eq!(INTERRUPT_WINDOW_END - INTERRUPT_WINDOW_START, MIB);
}

#[test]
fn giving_back_moves_the_pointer_only_for_the_top_grant() {
    let (a, a_end) = place(IOVA_BASE, PAGE).unwrap();
    let (b, b_end) = place(a_end, 2 * PAGE).unwrap();
    assert_eq!(after_give_back(b_end, b, 2 * PAGE), b);
    assert_eq!(after_give_back(b_end, a, PAGE), b_end);
    assert_eq!(after_give_back(a_end, a, PAGE), a);
    assert_eq!(after_give_back(b_end, u64::MAX, 2), b_end);
    // A grant that jumped the window gives back to where it started, above it.
    let below = INTERRUPT_WINDOW_START - PAGE;
    let (c, c_end) = place(below, 2 * PAGE).unwrap();
    assert_eq!(c, INTERRUPT_WINDOW_END);
    let next = after_give_back(c_end, c, 2 * PAGE);
    assert_eq!(next, INTERRUPT_WINDOW_END);
    assert!(place(next, PAGE).is_some_and(|(s, _)| !touches_interrupt_window(s, PAGE)));
}
