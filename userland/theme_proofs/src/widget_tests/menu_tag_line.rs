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

//! Menu: once any row has a tag, each tag starts where the untagged menu
//! starts its label, every label starts at one x, the widest tag and
//! `SPACE.md` past that, and the menu widens by as much, up to its cap. No
//! tag reaches its label in any box, at any scale.

use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::design::size::{MENU_MAX_W, SPACE};
use nonos_toolkit::widgets::{text, Item, Menu};

use super::boxes::boxes;
use super::menu::MENUS;
use super::menu_tag::{rows, tagged, TAG_SETS};
use super::serial::{serial, SCALES};

/// What is wrong with `m`, `items` tagged with `tags`, drawn at its size
/// beside `items` untagged at theirs.
fn misplaced(items: &[Item<'static>], tags: &[&'static str]) -> Option<String> {
    let all = tagged(items, tags);
    let (plain, m) = (Menu::new(items), Menu::new(&all));
    let (p, t) = (rows(plain, boxes(plain.size())[0]), rows(m, boxes(m.size())[0]));
    let px = t.first()?[0].px;
    let live = (0..items.len()).filter(|&i| items[i] != Item::separator());
    let widest = live.map(|i| text::width(tags[i % tags.len()], px)).max()?;
    let room = widest + scale().px(u32::from(SPACE.md));
    let wide = (plain.size().0 + room).min(scale().px(MENU_MAX_W));
    let starts = p.iter().zip(&t).all(|(p, t)| p[0].x == t[0].x);
    let shifted = m.size().0 == scale().px(MENU_MAX_W) || t[0][1].x == p[0][0].x + room as i32;
    (m.size() != (wide, plain.size().1) || !starts || !shifted || p.len() != t.len())
        .then(|| format!("{:?} against {:?}: {p:?} {t:?}", m.size(), plain.size()))
}

#[test]
fn menu_tags_line_up_every_label() {
    let _g = serial();
    let mut fails = Vec::new();
    for q in SCALES {
        set_scale(q);
        for (name, items) in MENUS {
            for tags in TAG_SETS {
                let all = tagged(items, tags);
                let m = Menu::new(&all);
                let what = |e: String| format!("{name} with {} tags at {q}/4: {e}", tags.len());
                fails.extend(misplaced(items, tags).map(what));
                for at in boxes(m.size()) {
                    let rs = rows(m, at);
                    let x = rs.first().map(|r| r[1].x);
                    let off = rs.iter().find(|r| r[0].end > r[1].x || Some(r[1].x) != x);
                    fails.extend(off.map(|r| what(format!("in {at:?}: {r:?}"))));
                }
            }
        }
    }
    set_scale(SCALES[0]);
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}
