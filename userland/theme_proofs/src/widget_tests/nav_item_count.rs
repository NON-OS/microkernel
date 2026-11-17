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

//! NavItem: a count ends `SPACE.md` short of the right edge, at least
//! `SPACE.md` past the label's ink, and widens the item by exactly its
//! measured width and that gap.

use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::design::size::SPACE;
use nonos_toolkit::design::text_role::TextRole;
use nonos_toolkit::icons::IconId;
use nonos_toolkit::widgets::{text, NavItem, Rect};

use super::boxes::boxes;
use super::canvas::Canvas;
use super::hide::Hide;
use super::mask::Masked;
use super::samples::LONG;
use super::serial::{serial, SCALES};

/// The columns `hide` covers when `n` is drawn in `at`, or None.
fn span(n: NavItem<'static>, at: Rect, hide: Hide) -> Option<(u32, u32)> {
    let lit = Canvas::new(at, 0).painted(|fb| n.draw(fb, at));
    let off = Canvas::new(at, 0).painted(|fb| n.draw(&mut Masked { fb, hide, rounds: 0 }, at));
    let xs = lit.diff(&off).into_iter().map(|p| p.0);
    xs.fold(None, |s, x| Some(s.map_or((x, x), |(a, b): (u32, u32)| (a.min(x), b.max(x)))))
}

#[test]
fn nav_item_count_trails_the_label_inside_the_body() {
    let _g = serial();
    let mut fails = Vec::new();
    for q in SCALES {
        set_scale(q);
        let pad = scale().px(u32::from(SPACE.md));
        for label in ["Tabs", LONG] {
            let plain = NavItem::new(label).icon(IconId::Files);
            let n = plain.count(1234);
            let digits = text::width("1234", scale().font(TextRole::Body.px())) + pad;
            if n.size() != (plain.size().0 + digits, plain.size().1) {
                fails.push(format!("{label} at {q}/4: {:?} from {:?}", n.size(), plain.size()));
            }
            for at in boxes(n.size()) {
                let count = span(n, at, Hide::Text("1234"));
                if count.is_none() && at.w < n.size().0 {
                    continue;
                }
                let text = span(n, at, Hide::Text(label));
                let (end, slack) = (at.x + at.w - pad, scale().px(2));
                let ok = count.zip(text).is_some_and(|((c0, c1), (_, t1))| {
                    t1 + pad <= c0 && c1 < end && c1 + 1 + slack >= end
                });
                if !ok {
                    fails.push(format!("{label} in {at:?} at {q}/4: {count:?} after {text:?}"));
                }
            }
        }
    }
    set_scale(SCALES[0]);
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}
