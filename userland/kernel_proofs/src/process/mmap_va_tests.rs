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

//! The mmap allocator: a guarded range has a page on each side that nothing
//! else is ever handed, plain ranges still meet end to end (the heap needs
//! them to), and every range, guards included, comes back on release.

use super::mmap_va::MmapVa;

const P: u64 = 4096;
const BASE: u64 = 0x0000_8000_0000;

#[test]
fn plain_ranges_meet_end_to_end_as_the_heap_needs() {
    let mut va = MmapVa::new();
    let a = va.reserve(4).expect("a");
    let b = va.reserve(4).expect("b");
    assert_eq!(a, BASE);
    assert_eq!(b, a + 4 * P);
}

#[test]
fn a_guarded_range_has_a_free_page_either_side() {
    let mut va = MmapVa::new();
    let a = va.reserve(2).expect("a");
    let g = va.reserve_guarded(3).expect("g");
    let b = va.reserve(2).expect("b");
    assert_eq!(g, a + 2 * P + P, "a page between the plain range and the view");
    assert_eq!(b, g + 3 * P + P, "and one after it");
}

#[test]
fn release_gives_back_the_guards_with_the_range() {
    let mut va = MmapVa::new();
    let g = va.reserve_guarded(3).expect("g");
    assert!(va.release(g, 3));
    // Everything came back, so the next plain range starts at the bottom.
    assert_eq!(va.reserve(5), Some(BASE));
}

#[test]
fn a_guarded_range_is_released_only_by_its_own_base_and_length() {
    let mut va = MmapVa::new();
    let g = va.reserve_guarded(2).expect("g");
    assert!(!va.release(g - P, 4), "the outer window is not a range it handed out");
    assert!(!va.release(g, 3));
    assert!(va.release(g, 2));
    assert!(!va.release(g, 2), "twice");
}

#[test]
fn churn_never_puts_a_range_on_a_guard_or_leaks() {
    let mut va = MmapVa::new();
    let mut held: Vec<(u64, u64, bool)> = Vec::new();
    let mut seed = 11u64;
    for _ in 0..20_000 {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let pages = (seed >> 33) % 5 + 1;
        if seed >> 63 == 1 && !held.is_empty() {
            let (b, n, _) = held.swap_remove((seed >> 8) as usize % held.len());
            assert!(va.release(b, n));
            continue;
        }
        let guarded = (seed >> 40) & 1 == 1;
        let base = if guarded { va.reserve_guarded(pages) } else { va.reserve(pages) };
        let b = base.expect("room");
        let (lo, hi) = (b, b + pages * P);
        for &(c, m, g) in &held {
            let (clo, chi) = (c, c + m * P);
            assert!(hi <= clo || chi <= lo, "overlap at {b:#x}");
            // Either side guarded: a page apart at least.
            if guarded || g {
                assert!(hi + P <= clo || chi + P <= lo, "no guard between {b:#x} and {c:#x}");
            }
        }
        held.push((b, pages, guarded));
    }
    for (b, n, _) in held.drain(..) {
        assert!(va.release(b, n));
    }
    assert_eq!(va.reserve(1), Some(BASE), "nothing leaked");
}

#[test]
fn a_view_release_gives_back_a_guarded_range_and_nothing_else() {
    let mut va = MmapVa::new();
    let own = va.reserve(4).expect("a mapping of the process's own");
    let view = va.reserve_guarded(4).expect("a surface view");
    assert!(!va.release_guarded(own, 4), "a plain range is never a view's window");
    assert!(!va.release_guarded(view + P, 3), "nor a part of one");
    assert!(!va.release_guarded(view, 5), "nor a longer one");
    assert!(va.release_guarded(view, 4));
    assert!(!va.release_guarded(view, 4), "twice");
    // The plain range is still held: the next one lands after it.
    assert_eq!(va.reserve(4), Some(own + 4 * P));
}

#[test]
fn views_attached_and_released_for_ever_reuse_one_window() {
    // A 1920x1080 buffer is 2025 pages: a compositor that views a client's
    // buffer and lets it go, frame after frame, holds one window, not one
    // for every view it ever made.
    let mut va = MmapVa::new();
    let first = va.reserve_guarded(2025).expect("view");
    assert!(va.release_guarded(first, 2025));
    for _ in 0..100_000 {
        let v = va.reserve_guarded(2025).expect("room");
        assert_eq!(v, first, "the same window each time");
        assert!(va.release_guarded(v, 2025));
    }
    assert_eq!(va.reserve(1), Some(BASE), "nothing left behind");
}
