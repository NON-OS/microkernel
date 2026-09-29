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

//! The lookup is a binary search, so each table must be sorted and its
//! ranges must not overlap, and the two halves of the zero table must meet
//! in order.

use super::wide::WIDE;
use super::zero_high::ZERO_HIGH;
use super::zero_low::ZERO_LOW;

fn sorted(t: &[(u32, u32)]) -> bool {
    t.iter().all(|&(lo, hi)| lo <= hi) && t.windows(2).all(|w| w[0].1 < w[1].0)
}

#[test]
fn tables_are_sorted_and_disjoint() {
    assert!(sorted(ZERO_LOW) && sorted(ZERO_HIGH) && sorted(WIDE));
    assert!(ZERO_LOW.last().unwrap().1 < ZERO_HIGH[0].0);
}

#[test]
fn known_widths() {
    use super::lookup::width;
    for (c, w) in
        [('a', 1), ('\u{301}', 0), ('漢', 2), ('한', 2), ('🚀', 2), ('\u{fe0f}', 0), ('│', 1)]
    {
        assert_eq!(width(c), w, "{c:?}");
    }
}
