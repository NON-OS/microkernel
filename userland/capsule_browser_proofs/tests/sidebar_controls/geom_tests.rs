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

use capsule_browser_proofs::browser::ui::rect::Rect;
use capsule_browser_proofs::browser::ui::sidebar_geom::{brand, dividers, rows};

use super::kit::{at_scale, side};

fn overlap(a: Rect, b: Rect) -> bool {
    a.x < b.right() && b.x < a.right() && a.y < b.bottom() && b.y < a.bottom()
}

#[test]
fn the_rail_has_square_rows_and_no_brand() {
    for q in [4, 5, 6, 8] {
        let _g = at_scale(q);
        for s in (300..=1400).step_by(7).map(|h| side(true, h)) {
            assert!(rows(true, s, 3).iter().all(|r| r.rect.w == r.rect.h), "at {q}");
            assert_eq!(brand(true, s).h, 0);
        }
        let b = brand(false, side(false, 700));
        assert!(b.h > 0 && b.w > 0 && b.y >= 56, "at {q}");
    }
}

#[test]
fn the_dividers_fall_between_the_rows() {
    for q in [4, 5, 6, 8] {
        let _g = at_scale(q);
        let tall = |r| (560..=1400).step_by(7).map(move |h| (r, h * q as i32 / 4));
        for (rail, h) in [false, true].into_iter().flat_map(tall) {
            let s = side(rail, h);
            let r = rows(rail, s, 2);
            for d in dividers(rail, s, 2) {
                assert!(d > s.y && d < s.bottom());
                let apart = r.iter().all(|o| d < o.rect.y || d >= o.rect.bottom());
                assert!(apart, "{rail} {s:?} {q}");
            }
        }
    }
}

#[test]
fn a_short_sidebar_pushes_settings_down_rather_than_over() {
    let short = |q| (0..300).flat_map(move |h| [(q, h, false), (q, h, true)]);
    for (q, h, rail) in [4, 5, 6, 8].into_iter().flat_map(short) {
        let _g = at_scale(q);
        let s = Rect::new(0, 56, if rail { 64 } else { 232 }, h);
        let r = rows(rail, s, 0);
        for w in r.windows(2) {
            assert!(!overlap(w[0].rect, w[1].rect), "{rail} h={h} at {q}");
            assert!(w[1].rect.y > w[0].rect.bottom(), "{rail} h={h} at {q}");
        }
    }
}
