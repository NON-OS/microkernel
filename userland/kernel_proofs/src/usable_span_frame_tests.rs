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

/*
 * The allocator reserves every frame a gap touches and counts as RAM the
 * frames the walk counts, so "in use" is RAM less free frames. That only
 * holds if each frame of the span is one or the other, never both and never
 * neither. Random maps with unaligned edges, reserved as the kernel's
 * reserve_range reserves them.
 */

use crate::memory::phys::usable_span::walk_usable;

const PAGE: u64 = 4096;
const SPAN: (u64, u64) = (0x10_0000, 0x40_0000);

fn xorshift(state: &mut u64) -> u64 {
    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *state = x;
    x
}

/* Every frame that overlaps `[a, b)` inside the span, as reserve_range marks it. */
fn reserve(taken: &mut [bool], a: u64, b: u64) {
    let first = a.max(SPAN.0) / PAGE * PAGE;
    let last = b.min(SPAN.1);
    if last > first {
        let from = ((first - SPAN.0) / PAGE) as usize;
        let to = (from + (last - first).div_ceil(PAGE) as usize).min(taken.len());
        taken[from..to].fill(true);
    }
}

#[test]
fn the_count_and_the_gaps_agree_on_every_frame() {
    let mut s = 0x2545_f491_4f6c_dd1du64;
    for _ in 0..20_000 {
        let n = xorshift(&mut s) % 7;
        let mut map: Vec<(u64, u64)> = (0..n)
            .map(|_| {
                let a = xorshift(&mut s) % 0x48_0000;
                (a, a + xorshift(&mut s) % 0x8_0000)
            })
            .collect();
        map.sort_unstable();
        let mut taken = vec![false; ((SPAN.1 - SPAN.0) / PAGE) as usize];
        let bytes = walk_usable(&map, SPAN.0, SPAN.1, |a, b| reserve(&mut taken, a, b));
        let free = taken.iter().filter(|t| !**t).count() as u64;
        assert_eq!(bytes, free * PAGE, "{map:x?}");
    }
}
