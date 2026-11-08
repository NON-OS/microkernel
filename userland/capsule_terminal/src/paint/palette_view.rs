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

//! The command palette as the shared `Menu` under a `Field`: the rows a query
//! keeps, where the two sit over the body, and which rows they light.

use alloc::vec::Vec;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::SPACE;
use nonos_toolkit::widgets::{Field, Item, Menu, Pointer, Rect};

use crate::palette::{filter, Entry, MAX_ROWS};

/// The entries `needle` keeps, in index order. Row `i` of the menu is entry
/// `i` here, for the painter, the pointer and the pick alike.
pub fn matches<'a>(entries: &[Entry<'a>], needle: &[u8]) -> Vec<Entry<'a>> {
    let mut hits = [0usize; MAX_ROWS];
    let n = filter(entries, needle, &mut hits);
    hits[..n].iter().map(|&i| entries[i]).collect()
}

/// One menu row per match: its kind's tag, its label, and its hint in the
/// shortcut column.
pub fn items<'a>(m: &[Entry<'a>]) -> Vec<Item<'a>> {
    m.iter().map(|e| Item::new(e.label).shortcut(e.hint).tag(e.kind.tag())).collect()
}

/// The row the selection names, clamped to the `n` matches: washed, and acted on.
pub fn chosen(n: usize, sel: usize) -> Option<usize> {
    n.checked_sub(1).map(|last| sel.min(last))
}

/// The query field, and the menu `SPACE.xs` under it with the rect it is
/// drawn in: three quarters of the body across, centred, `SPACE.xl` from its
/// top, and cut to its bottom. The menu holds the `rows` items that fit.
pub struct View<'a> {
    pub field: Rect,
    pub menu: Menu<'a>,
    pub at: Rect,
    pub rows: usize,
}

pub fn view<'a>(body: Rect, items: &'a [Item<'a>]) -> View<'a> {
    let (s, bottom) = (scale(), body.y.saturating_add(body.h));
    let edge = s.px(u32::from(SPACE.xl)).min(body.w / 2);
    let w = (body.w * 3 / 4).min(body.w - edge * 2);
    let y = body.y.saturating_add(s.px(u32::from(SPACE.xl))).min(bottom);
    let h = Field::new("").size().1.min(bottom - y);
    let field = Rect { x: body.x + (body.w - w) / 2, y, w, h };
    let top = (y + h).saturating_add(s.px(u32::from(SPACE.xs))).min(bottom);
    let fit = |m: Menu| Rect { y: top, h: m.size().1.min(bottom - top), ..field };
    let rows = Menu::new(items).shown(fit(Menu::new(items)));
    let menu = Menu::new(&items[..rows]).width(w);
    View { field, menu, at: fit(menu), rows }
}

/// The row washed, the selection, and the row a held press began on and stays on.
pub fn lit(v: &View, ptr: &Pointer, sel: usize) -> (Option<usize>, Option<usize>) {
    let row = |x: i32, y: i32| v.menu.hit(v.at, x, y);
    let under = ptr.position().and_then(|(x, y)| row(x, y));
    let held = under.filter(|&i| ptr.state(|x, y| row(x, y) == Some(i)).pressed);
    (chosen(v.rows, sel), held)
}
