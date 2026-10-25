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

//! Random maps and frees against the kernel's own take order (a hole first,
//! then fresh space): no two live grants ever overlap, the hole list keeps its
//! shape, and freeing everything gives every byte back.

use super::holes::{give_back, take_hole, Holes};
use super::space::{place, touches_interrupt_window, IOVA_BASE};

const PAGE: u64 = 4096;

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

fn take(next: &mut u64, holes: &mut Holes, length: u64) -> u64 {
    if let Some(start) = take_hole(holes, length) {
        return start;
    }
    let (start, end) = place(*next, length).unwrap();
    *next = end;
    start
}

fn check(next: u64, holes: &Holes, live: &[(u64, u64)]) {
    for w in holes.windows(2) {
        assert!(w[0].1 < w[1].0, "holes sorted, apart: {holes:x?}");
    }
    let mut all: Vec<(u64, u64)> = live.iter().map(|&(s, l)| (s, s + l)).collect();
    all.extend(holes.iter().copied());
    all.sort();
    for w in all.windows(2) {
        assert!(w[0].1 <= w[1].0, "overlap: {:x?} {:x?}", w[0], w[1]);
    }
    assert!(all.iter().all(|&(s, e)| s >= IOVA_BASE && s < e && e <= next));
    assert!(live.iter().all(|&(s, l)| !touches_interrupt_window(s, l)));
}

#[test]
fn random_churn_never_overlaps_and_returns_everything() {
    let mut seed = 0x9E37_79B9_7F4A_7C15u64;
    for _round in 0..64 {
        let (mut next, mut holes) = (IOVA_BASE, Holes::new());
        let mut live: Vec<(u64, u64)> = Vec::new();
        for _ in 0..400 {
            let r = xorshift(&mut seed);
            if live.len() < 48 && (r & 3 != 0 || live.is_empty()) {
                let length = (1 + (r >> 8) % 16) * PAGE;
                live.push((take(&mut next, &mut holes, length), length));
            } else {
                let (s, l) = live.swap_remove((r >> 8) as usize % live.len());
                give_back(&mut next, &mut holes, s, l);
            }
            check(next, &holes, &live);
        }
        while !live.is_empty() {
            let i = xorshift(&mut seed) as usize % live.len();
            let (s, l) = live.swap_remove(i);
            give_back(&mut next, &mut holes, s, l);
            check(next, &holes, &live);
        }
        assert_eq!((next, holes.len()), (IOVA_BASE, 0));
    }
}
