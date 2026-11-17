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

//! NavItem: a compact item is a `NAV_H` square showing only its icon,
//! centred in the body, in every box, with no text at all.

use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::design::size::{ICON, NAV_H};
use nonos_toolkit::icons::IconId;
use nonos_toolkit::widgets::NavItem;

use super::boxes::{boxes, within};
use super::row_marks::Marks;
use super::serial::{serial, SCALES};

#[test]
fn nav_item_compact_centres_its_icon_alone() {
    let _g = serial();
    let mut fails = Vec::new();
    for q in SCALES {
        set_scale(q);
        let n = NavItem::new("Network").icon(IconId::SettingsNetwork).count(4).compact();
        let side = scale().px(NAV_H);
        if n.size() != (side, side) {
            fails.push(format!("size {:?} at {q}/4", n.size()));
        }
        for at in boxes(n.size()) {
            let mut m = Marks::default();
            n.draw(&mut m, at);
            let Some(&sq) = m.1.first() else {
                fails.push(format!("no icon in {at:?} at {q}/4"));
                continue;
            };
            let gap = |a: u32, n: u32, b: u32, m: u32| (b + m - (a + n)).abs_diff(a - b);
            let centred = gap(sq.x, sq.w, at.x, at.w) <= 1 && gap(sq.y, sq.h, at.y, at.h) <= 1;
            let whole = sq.w == scale().px(ICON).min(at.h).min(at.w);
            if !m.0.is_empty() || m.1.len() != 1 || !centred || !whole || !within(sq, at) {
                fails.push(format!("in {at:?} at {q}/4: {sq:?} with text {:?}", m.0));
            }
        }
    }
    set_scale(SCALES[0]);
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}
