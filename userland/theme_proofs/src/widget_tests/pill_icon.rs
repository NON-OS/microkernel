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

//! Pill: an icon sits `PILL_PAD` in from the left, inside the body, the
//! label starts `SPACE.xs` past it, it takes the dot's place, and it widens
//! the pill by exactly its square and gap.

use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::design::size::{ICON, PILL_PAD, SPACE};
use nonos_toolkit::icons::IconId;
use nonos_toolkit::theme::Category;
use nonos_toolkit::widgets::rect::centre;
use nonos_toolkit::widgets::{Pill, Rect, Tone};

use super::boxes::boxes;
use super::canvas::Canvas;
use super::hide::Hide;
use super::mask::Masked;
use super::samples::LONG;
use super::serial::{serial, SCALES};

/// The columns and rows `hide` covers when `p` is drawn in `at`, or None.
fn ink(p: Pill<'static>, at: Rect, hide: Hide) -> Option<(u32, u32, u32, u32)> {
    let lit = Canvas::new(at, 0).painted(|fb| p.draw(fb, at));
    let off = Canvas::new(at, 0).painted(|fb| p.draw(&mut Masked { fb, hide, rounds: 0 }, at));
    let grow = |s: Option<(u32, u32, u32, u32)>, (x, y): (u32, u32)| {
        Some(s.map_or((x, x, y, y), |(a, b, c, d)| (a.min(x), b.max(x), c.min(y), d.max(y))))
    };
    lit.diff(&off).into_iter().fold(None, grow)
}

#[test]
fn pill_icon_leads_the_label_inside_the_body() {
    let _g = serial();
    let mut fails = Vec::new();
    for (q, label) in SCALES.into_iter().flat_map(|q| [(q, "Saved"), (q, LONG)]) {
        set_scale(q);
        let (s, bare, k) = (scale(), Pill::new(label), Category::new(3));
        let (side, gap, pad) = (s.px(ICON), s.px(u32::from(SPACE.xs)), s.px(PILL_PAD));
        let plain = [bare, bare.tone(Tone::Ok), bare.filled(k), bare.count(1234)];
        for (plain, dotted) in plain.into_iter().flat_map(|p| [(p, p), (p, p.dot())]) {
            let p = dotted.icon(IconId::Shield);
            if p.size() != (plain.size().0 + side + gap, plain.size().1) {
                fails.push(format!("{p:?} at {q}/4: {:?} from {:?}", p.size(), plain.size()));
            }
            for at in boxes(p.size()).into_iter().filter(|at| at.w >= p.size().0) {
                let body = centre(at, p.size().0, p.size().1);
                let (lead, start) = (at.x + pad, at.x + pad + side + gap);
                let (icon, text) = (ink(p, at, Hide::Icons), ink(p, at, Hide::Text(label)));
                let ok = icon.zip(text).is_some_and(|((i0, i1, y0, y1), (t0, ..))| {
                    i0 >= lead && i1 < lead + side && y0 >= body.y && y1 < body.y + body.h
                        && t0 + s.px(2) >= start && t0 <= start + s.px(2)
                });
                if !ok || ink(p, at, Hide::Dots).is_some() {
                    fails.push(format!("{p:?} in {at:?} at {q}/4: icon {icon:?} text {text:?}"));
                }
            }
        }
    }
    set_scale(SCALES[0]);
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}
