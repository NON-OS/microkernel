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

//! The canvas a screen gets and the bytes behind it, at every edge.

use super::canvas_floor::{backing, floor_canvas};
use super::canvas_scale::scale_for_panel;
use super::edges::{pairs, EDGES};
use crate::display::scale_rule::canvas_for;

fn floor_model(w: u32, h: u32) -> Option<(u32, u32)> {
    if w == 0 || h == 0 || (w >= 1024 && h >= 720) {
        return None;
    }
    let (w, h) = (w as u128, h as u128);
    let (num, den) = if 1024 * h >= 720 * w { (1024, w) } else { (720, h) };
    let side = |v: u128| u32::try_from((v * num).div_ceil(den)).ok();
    Some((side(w)?, side(h)?))
}

#[test]
fn the_floor_canvas_and_its_backing_are_exact_or_refused_at_every_edge() {
    let mut refused = 0;
    for (w, h) in pairs() {
        assert_eq!(floor_canvas(w, h), floor_model(w, h), "{w}x{h}");
        let stride = w as u128 * 4;
        let want =
            (stride <= u32::MAX as u128).then(|| (stride as u32, (stride * h as u128) as u64));
        assert_eq!(backing(w, h), want, "{w}x{h}");
        refused += want.is_none() as u32;
        let _ = scale_for_panel(w, h, Some((w, h)));
        let _ = scale_for_panel(w, h, None);
    }
    assert!(refused > 100, "{refused}");
}

#[test]
fn a_scaled_canvas_is_never_larger_than_its_screen_at_every_edge() {
    for ((w, h), q) in pairs().flat_map(|p| EDGES.iter().map(move |&q| (p, q))) {
        let q4 = q.max(4) as u128;
        let want = ((w as u128 * 4 / q4) as u32, (h as u128 * 4 / q4) as u32);
        let got = canvas_for(w, h, q);
        assert_eq!(got, want, "{w}x{h} q {q}");
        assert!(got.0 <= w && got.1 <= h);
    }
}
