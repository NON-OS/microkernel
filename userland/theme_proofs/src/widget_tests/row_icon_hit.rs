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
//! Row: an icon moves no hit and touches no pixel outside the text lane. A
//! row with an icon and the same row without one are one size, every point
//! in and round every box hits both or neither, and every pixel they differ
//! in lies between where the plain label starts and the trailing box, in
//! every look, at every scale.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::icons::IconId;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{Rect, Row};

use super::boxes::boxes;
use super::canvas::Canvas;
use super::row_marks::marks;
use super::serial::{serial, SCALES};
use super::states::looks;

/// Every point of `at` and 4 px round it.
fn around(at: Rect) -> impl Iterator<Item = (i32, i32)> {
    let (x0, y0) = (at.x as i32 - 4, at.y as i32 - 4);
    let ys = y0..y0 + at.h as i32 + 8;
    ys.flat_map(move |y| (x0..x0 + at.w as i32 + 8).map(move |x| (x, y)))
}

/// `r` drawn in `at` on the surface ground.
fn drawn(r: Row<'static>, at: Rect) -> Canvas {
    Canvas::new(at, roles().surface.argb()).painted(|fb| r.draw(fb, at))
}

#[test]
fn row_icon_keeps_every_hit_and_pixel_outside_the_lane() {
    let _g = serial();
    let mut fails = Vec::new();
    for q in SCALES {
        set_scale(q);
        let base = Row::new("Wi-Fi").note("Connected").value("Home").trailing(40).chevron();
        for st in looks() {
            let (plain, lit) = (base.state(st), base.icon(IconId::FsFile).state(st));
            for at in boxes(plain.size()) {
                let what = format!("{st:?} in {at:?} at {q}/4");
                if plain.size() != lit.size() {
                    fails.push(format!("{what}: {:?} against {:?}", lit.size(), plain.size()));
                }
                if let Some(p) = around(at).find(|&(x, y)| plain.hit(at, x, y) != lit.hit(at, x, y)) {
                    fails.push(format!("{what}: hit moved at {p:?}"));
                }
                let (from, to) = (marks(plain, at).0[0].0 as u32, plain.trailing_rect(at).x);
                let diff = drawn(lit, at).diff(&drawn(plain, at));
                let stray = diff.iter().find(|&&(x, y)| x < from || x >= to || y < at.y || y >= at.y + at.h);
                fails.extend(stray.map(|p| format!("{what}: icon row differs at {p:?}")));
            }
        }
    }
    set_scale(SCALES[0]);
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}
