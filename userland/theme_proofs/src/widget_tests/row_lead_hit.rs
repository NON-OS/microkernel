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
//! Row: a leading box moves no hit and is the app's own. A row with one and
//! the same row without one are one size and hit at every point in and round
//! every box; a pressed, selected or hovered wash covers every pixel of the
//! box, and a row with no wash and no focus ring leaves it untouched, in
//! every look, at every scale.

use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{Rect, Row};

use super::boxes::{boxes, inside};
use super::canvas::Canvas;
use super::serial::{serial, SCALES};
use super::states::looks;

/// Every point of `at` and 4 px round it.
fn around(at: Rect) -> impl Iterator<Item = (i32, i32)> {
    let (x0, y0) = (at.x as i32 - 4, at.y as i32 - 4);
    let ys = y0..y0 + at.h as i32 + 8;
    ys.flat_map(move |y| (x0..x0 + at.w as i32 + 8).map(move |x| (x, y)))
}

#[test]
fn row_lead_keeps_every_hit_and_takes_the_washes() {
    let _g = serial();
    let mut fails = Vec::new();
    for q in SCALES {
        set_scale(q);
        let base = Row::new("Wi-Fi").note("Connected").value("Home").trailing(40).chevron();
        for st in looks() {
            let (plain, lead) = (base.state(st), base.leading(scale().px(24)).state(st));
            let washed = !st.disabled && (st.pressed || st.selected || st.hovered);
            for at in boxes(plain.size()) {
                let what = format!("{st:?} in {at:?} at {q}/4");
                if plain.size() != lead.size() {
                    fails.push(format!("{what}: {:?} against {:?}", lead.size(), plain.size()));
                }
                if let Some(p) = around(at).find(|&(x, y)| plain.hit(at, x, y) != lead.hit(at, x, y)) {
                    fails.push(format!("{what}: hit moved at {p:?}"));
                }
                let (ground, b) = (roles().surface.argb(), lead.leading_rect(at));
                let changed = Canvas::new(at, ground).painted(|fb| lead.draw(fb, at)).changed(ground);
                let inked = changed.iter().filter(|&&p| inside(b, p)).count() as u32;
                if washed && inked != b.w * b.h {
                    fails.push(format!("{what}: wash covers {inked} of {b:?}"));
                }
                if !washed && !st.focused && inked != 0 {
                    fails.push(format!("{what}: drew {inked} pixels in {b:?}"));
                }
            }
        }
    }
    set_scale(SCALES[0]);
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}
