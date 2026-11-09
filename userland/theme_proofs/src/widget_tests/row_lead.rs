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
//! Row: a leading box sits at the row's left padding, the row's full height
//! and inside the body, and the label, or an icon before it, starts
//! `SPACE.md` past it. A row without one has a zero-wide box where its label
//! starts. Read back from what each row draws, at every scale and in every box.

use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::design::size::SPACE;
use nonos_toolkit::icons::IconId;
use nonos_toolkit::widgets::{Rect, Row};

use super::boxes::{boxes, within};
use super::row_marks::marks;
use super::serial::{serial, SCALES};

/// The text or icon the row draws first, left to right.
fn first(r: Row<'static>, at: Rect) -> i32 {
    let m = marks(r, at);
    m.1.first().map_or(m.0[0].0, |sq| sq.x as i32)
}

#[test]
fn row_lead_box_is_at_the_padding_and_the_label_follows_it() {
    let _g = serial();
    let mut fails = Vec::new();
    for q in SCALES {
        set_scale(q);
        let s = scale();
        let (pad, gap, w) = (s.px(u32::from(SPACE.lg)), s.px(u32::from(SPACE.md)), s.px(24));
        let plain = Row::new("Wi-Fi");
        let rows = [plain, plain.note("Connected"), plain.value("Home")];
        for r in rows.into_iter().flat_map(|r| [r, r.icon(IconId::FsFile)]) {
            for at in boxes(r.size()) {
                let what = format!("in {at:?} at {q}/4");
                let none = r.leading_rect(at);
                if none.w != 0 || none.x != at.x + pad || first(r, at) != none.x as i32 {
                    fails.push(format!("{what}: no leading box gave {none:?}"));
                }
                let lead = r.leading(w);
                let b = lead.leading_rect(at);
                let placed = b.x == at.x + pad && b.w == w && b.y == at.y && b.h == at.h;
                if !placed || !within(b, at) {
                    fails.push(format!("{what}: leading box {b:?}"));
                }
                let start = first(lead, at);
                if start != (b.x + b.w + gap) as i32 {
                    fails.push(format!("{what}: label or icon at {start} after {b:?}"));
                }
            }
        }
    }
    set_scale(SCALES[0]);
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}
