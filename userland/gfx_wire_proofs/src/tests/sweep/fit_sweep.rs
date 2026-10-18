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

//! The mode asked for from what a driver claims (any range, alignment,
//! budget or list): never a panic or a wrap; a range answer inside the
//! range, aligned and in budget; a list answer the largest listed mode
//! inside the wanted size; never the size already set.

use super::rng::Rng;
use crate::body::caps::{Caps, MODE_RANGE};
use crate::body::mode::Mode;
use crate::fit::fit;

const SLOT: u128 = 64 * 64 * 4;

fn big(r: &mut Rng) -> u32 {
    [r.word(), r.below(9000), u32::MAX - r.below(4)][r.below(3) as usize]
}

/// A driver's claimed range, alignment, budget and list.
fn claim(r: &mut Rng) -> (Caps, Vec<Mode>) {
    let mut c = Caps { outputs: 1, ..Caps::default() };
    c.flags = if r.below(2) == 0 { MODE_RANGE } else { 0 };
    c.surface_budget = [r.next(), r.next() % (1 << 26), u64::MAX][r.below(3) as usize];
    (c.max_width, c.max_height, c.width_align) = (big(r), big(r), r.word() as u16);
    (c, (0..r.below(24)).map(|_| Mode { width: big(r), height: big(r), flags: 0 }).collect())
}

/// Inside the claimed range, aligned, and the frame within the budget.
fn takes(c: &Caps, w: u32, h: u32) -> bool {
    let fits = (640..=c.max_width).contains(&w) && (480..=c.max_height).contains(&h);
    let frame = w as u128 * h as u128 * 4 + SLOT;
    fits && w.is_multiple_of((c.width_align as u32).max(1)) && frame <= c.surface_budget as u128
}

#[test]
fn a_driver_that_lies_in_its_range_or_list_gets_a_size_it_can_take() {
    let mut r = Rng::new(0xF17_5E1);
    let mut seen = [0usize; 3];
    for _ in 0..200_000 {
        let (want, current, (caps, modes)) =
            ((big(&mut r), big(&mut r)), (big(&mut r), big(&mut r)), claim(&mut r));
        let got = fit(want, current, &caps, &modes);
        assert!(got.is_none() || (want.0 > 0 && want.1 > 0), "a zero size asks nothing");
        let Some((w, h)) = got else {
            seen[0] += 1;
            continue;
        };
        assert_ne!((w, h), current, "the size already set is never asked again");
        if caps.has(MODE_RANGE) {
            assert!(takes(&caps, w, h), "{caps:?} {w}x{h}");
            seen[1] += 1;
        } else {
            let inside = modes.iter().take(20).filter(|m| m.width <= want.0 && m.height <= want.1);
            let best = inside.map(|m| (m.width as u64 * m.height as u64, m.width)).max();
            assert_eq!(Some((w as u64 * h as u64, w)), best, "the largest listed mode inside");
            seen[2] += 1;
        }
    }
    assert!(seen.iter().all(|&n| n >= 1_000), "[none, range, list] {seen:?}");
}
