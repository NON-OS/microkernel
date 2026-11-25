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

//! The toolkit's integer square root is the root the Newton loop it replaced
//! gave, for every value a corner's coverage asks and across the whole u64
//! range: every value to 2^22, every perfect square to 2^32 and its two
//! neighbours, the edges of the range, and a million values spread over it.
//! At u64::MAX, where the old loop overflowed, the root is checked as known.

use nonos_toolkit::paint::radius::{coverage, isqrt};

/// The root as it was taken before: Newton's loop, a division a step.
fn newton(v: u64) -> u32 {
    if v == 0 {
        return 0;
    }
    let mut x = v;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + v / x) / 2;
    }
    x as u32
}

#[test]
fn every_small_value_has_the_old_root() {
    for v in 0..1u64 << 22 {
        assert_eq!(isqrt(v), newton(v), "{v}");
    }
}

#[test]
fn every_square_and_its_neighbours_have_the_old_root() {
    for k in (0u64..1 << 32).step_by(997).chain([1 << 16, (1 << 32) - 1]) {
        let s = k * k;
        for v in [s.saturating_sub(1), s, s.saturating_add(1)] {
            assert_eq!(isqrt(v), newton(v), "{v}");
        }
    }
}

#[test]
fn values_across_the_range_have_the_old_root() {
    let mut v = 0x9E37_79B9_7F4A_7C15u64;
    for _ in 0..1_000_000 {
        v ^= v << 13;
        v ^= v >> 7;
        v ^= v << 17;
        let n = v >> (v % 64);
        assert_eq!(isqrt(n), newton(n), "{n}");
    }
    for n in [u64::MAX - 1, 1 << 63, (1 << 63) - 1] {
        assert_eq!(isqrt(n), newton(n), "{n}");
    }
}

#[test]
fn the_top_of_the_range_has_its_root() {
    // The old loop's first step, (v + 1) / 2, overflowed here and its next
    // divided by zero; no corner asks a value near it.
    assert_eq!(isqrt(u64::MAX), u32::MAX);
}

#[test]
fn a_corner_is_covered_as_before_at_every_radius() {
    // coverage() as it was, with the old root.
    fn old(px: u32, py: u32, w: u32, h: u32, r: u32) -> u32 {
        let off = |p: u32, span: u32| {
            let c = p * 256 + 128;
            let (near, far) = ((r * 256).saturating_sub(c), (r * 256 + c).saturating_sub(span * 256));
            near.max(far) as u64
        };
        if r == 0 || px >= w || py >= h {
            return 255;
        }
        let (dx, dy) = (off(px, w), off(py, h));
        if dx == 0 || dy == 0 {
            return 255;
        }
        (r * 256 + 128).saturating_sub(newton(dx * dx + dy * dy)).min(255)
    }
    for r in 0..=96 {
        let (w, h) = (2 * r + 7, 2 * r + 3);
        for py in 0..h {
            for px in 0..w {
                assert_eq!(coverage(px, py, w, h, r), old(px, py, w, h, r), "{px},{py} r {r}");
            }
        }
    }
}
