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

//! A colour-blind filter does its job: red and green, which the dichromat's
//! own simulation brings close, are further apart for that viewer once
//! filtered, through the look's real tables.

use crate::display::access_rule::{FILTER_DEUTAN, FILTER_PROTAN};
use crate::display::cvd_filter::{simulate, Cvd};
use crate::display::cvd_rule::apply as mix;
use crate::look_tests::{looked, rgb};

#[test]
fn red_and_green_move_apart_for_the_viewer() {
    let (red, green) = (0xFFD0_3020, 0xFF40_A030);
    for (f, kind) in [(FILTER_PROTAN, Cvd::Protan), (FILTER_DEUTAN, Cvd::Deutan)] {
        let s = simulate(kind).unwrap();
        let seen = |p: u32| mix(&s, rgb(p));
        let gap = |a: [f32; 3], b: [f32; 3]| {
            a.iter().zip(b.iter()).map(|(x, y)| (x - y).powi(2)).sum::<f32>()
        };
        let out = looked(f, &[red, green]);
        let before = gap(seen(red), seen(green));
        let after = gap(seen(out[0]), seen(out[1]));
        assert!(after > before * 1.2, "{kind:?}: {before} to {after}");
    }
}
