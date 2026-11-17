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

//! Pill: a count sits `SPACE.sm` past the label's ink, ends `PILL_PAD` in
//! from the right edge, and widens the pill by exactly its width and gap.
//! A short label cut to `…` in a narrow box is left to the bounds proof.

use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::design::size::{PILL_PAD, SPACE};
use nonos_toolkit::design::text_role::TextRole;
use nonos_toolkit::theme::Category;
use nonos_toolkit::widgets::{text, Pill, Rect};

use super::boxes::boxes;
use super::canvas::Canvas;
use super::hide::Hide;
use super::mask::Masked;
use super::samples::LONG;
use super::serial::{serial, SCALES};

/// The columns `hide` covers when `p` is drawn in `at`, or None.
fn span(p: Pill<'static>, at: Rect, hide: Hide) -> Option<(u32, u32)> {
    let lit = Canvas::new(at, 0).painted(|fb| p.draw(fb, at));
    let off = Canvas::new(at, 0).painted(|fb| p.draw(&mut Masked { fb, hide, rounds: 0 }, at));
    let xs = lit.diff(&off).into_iter().map(|p| p.0);
    xs.fold(None, |s, x| Some(s.map_or((x, x), |(a, b): (u32, u32)| (a.min(x), b.max(x)))))
}

#[test]
fn pill_count_trails_the_label_inside_the_body() {
    let _g = serial();
    let mut fails = Vec::new();
    for (q, label) in SCALES.into_iter().flat_map(|q| [(q, "Saved"), (q, LONG)]) {
        set_scale(q);
        let s = scale();
        let (gap, pad, slack) = (s.px(u32::from(SPACE.sm)), s.px(PILL_PAD), s.px(2));
        let digits = text::width("1234", s.font(TextRole::Body.px())) + gap;
        let (bare, k) = (Pill::new(label), Category::new(3));
        for plain in [bare, bare.category(k), bare.filled(k)] {
            let p = plain.count(1234);
            if p.size() != (plain.size().0 + digits, plain.size().1) {
                fails.push(format!("{p:?} at {q}/4: {:?} from {:?}", p.size(), plain.size()));
            }
            for at in boxes(p.size()) {
                let count = span(p, at, Hide::Text("1234"));
                if (count.is_none() || label != LONG) && at.w < p.size().0 {
                    continue;
                }
                let text = span(p, at, Hide::Text(label));
                let end = at.x + at.w.min(p.size().0) - pad;
                let ok = count.zip(text).is_some_and(|((c0, c1), (_, t1))| {
                    t1 + gap <= c0 && c1 < end && c1 + 1 + slack >= end
                });
                if !ok {
                    fails.push(format!("{p:?} in {at:?} at {q}/4: {count:?} after {text:?}"));
                }
            }
        }
    }
    set_scale(SCALES[0]);
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}
