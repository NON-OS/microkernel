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

//! Menu: an indented item's tag and label sit exactly `SPACE.sm` in from
//! where they sit unindented and stay in the row, rows left alone do not
//! move, the menu grows by the indent at most, and no hit moves.

use nonos_toolkit::design::{scale::{scale, set_scale}, size::{MENU_MAX_W, SPACE}};
use nonos_toolkit::widgets::{Item, Menu, Rect};

use super::boxes::boxes;
use super::menu::{MENUS, RULES};
use super::{menu_tag::{rows, tagged, Run}, menu_tag_hit::around};
use super::serial::{serial, SCALES};

/// `items` with every other item from the first indented.
fn indented(items: &[Item<'static>]) -> Vec<Item<'static>> {
    items.iter().enumerate().map(|(i, it)| if i % 2 == 0 { it.indent() } else { *it }).collect()
}

/// A line if a drawn run of `t` but the shortcut is not `p`'s moved by `by`.
fn moved(p: &[Run], t: &[Run], by: i32, at: Rect) -> Option<String> {
    let off = p.len() != t.len() || t.iter().any(|r| r.end > (at.x + at.w) as i32);
    let ink = |(p, t): &(&Run, &Run)| p.end > p.x && t.end > t.x;
    let mut lane = p.iter().zip(t).take(t.len().saturating_sub(1)).filter(ink);
    (off || !lane.all(|(p, t)| t.x == p.x + by)).then(|| format!("by {by} in {at:?}: {p:?} {t:?}"))
}

#[test]
fn menu_indent_shifts_only_the_indented_lane() {
    let _g = serial();
    let mut fails = Vec::new();
    for q in SCALES {
        set_scale(q);
        let step = scale().px(u32::from(SPACE.sm));
        for (name, items) in MENUS {
            for tags in [&[""][..], &["cmd", "日本"]] {
                let (base, all) = (tagged(items, tags), indented(&tagged(items, tags)));
                let each: Vec<_> = base.iter().map(|it| it.indent()).collect();
                let (m, plain) = (Menu::new(&all), Menu::new(&base));
                let want = (plain.size().0 + step).min(scale().px(MENU_MAX_W));
                let full = Menu::new(&each).size().0 == want || items == &RULES[..];
                let bad = m.size().0 > want || m.size().0 < plain.size().0 || !full;
                fails.extend(bad.then(|| format!("{name} at {q}/4: {:?}", m.size())));
                let plain = plain.width(m.size().0);
                for at in boxes(m.size()) {
                    let hit = around(at).find(|&(x, y)| plain.hit(at, x, y) != m.hit(at, x, y));
                    fails.extend(hit.map(|p| format!("{name} at {q}/4 in {at:?}: hit {p:?}")));
                    let (p, t) = (rows(plain, at), rows(m, at));
                    let live = (0..items.len()).filter(|&i| items[i] != Item::separator());
                    for (r, i) in live.enumerate().take(p.len().min(t.len())) {
                        let by = if i % 2 == 0 { step as i32 } else { 0 };
                        let e = moved(&p[r], &t[r], by, at);
                        fails.extend(e.map(|e| format!("{name} {i} at {q}/4: {e}")));
                    }
                }
            }
        }
    }
    set_scale(SCALES[0]);
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}
