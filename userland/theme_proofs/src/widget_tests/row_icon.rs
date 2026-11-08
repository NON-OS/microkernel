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
//! Row: an icon leads the label. The icon is an `ICON` square at the left of
//! the text lane, inside the body, and the label and note start `SPACE.md`
//! past it, where a row without an icon starts them `ICON` plus `SPACE.md`
//! further left. Read back from what each row draws, at every scale and in
//! every box. A row with too little lane for the gap is the bounds proof's.

use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::design::size::{ICON, SPACE};
use nonos_toolkit::icons::IconId;
use nonos_toolkit::widgets::Row;

use super::boxes::{boxes, within};
use super::row_marks::marks;
use super::serial::{serial, SCALES};

#[test]
fn row_icon_leads_the_label_by_icon_and_gap() {
    let _g = serial();
    let mut fails = Vec::new();
    for q in SCALES {
        set_scale(q);
        let (side, gap) = (scale().px(ICON), scale().px(u32::from(SPACE.md)) as i32);
        let plain = Row::new("Wi-Fi").note("Connected");
        for at in boxes(plain.size()) {
            let (p, t) = (marks(plain, at), marks(plain.icon(IconId::FsFile), at));
            let Some(&sq) = t.1.first() else {
                fails.push(format!("no icon in {at:?} at {q}/4"));
                continue;
            };
            let lead = sq.x as i32 == p.0[0].0 && sq.w == side && within(sq, at);
            let shift = side as i32 + gap;
            let text = (0..2).all(|i| t.0[i] == (p.0[i].0 + shift, p.0[i].1));
            if !lead || !text || t.0.len() != 2 || !p.1.is_empty() {
                fails.push(format!("in {at:?} at {q}/4: {:?} {:?} against {:?}", t.0, sq, p.0));
            }
        }
    }
    set_scale(SCALES[0]);
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}
