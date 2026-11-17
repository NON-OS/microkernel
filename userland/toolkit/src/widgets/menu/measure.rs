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

//! A menu's size, and the measures its rows share.

use super::{Item, Menu};
use crate::design::scale::scale;
use crate::design::size::{ICON, MENU_MAX_W, MENU_ROW_H, SPACE};
use crate::design::text_role::TextRole;
use crate::widgets::text;

impl Menu<'_> {
    /// Across: `SPACE.xs` and `SPACE.md` each side, the tick's room when any
    /// item has one, the tag column's room when any item has a tag, the
    /// widest label with its indent and, when any item has a shortcut,
    /// `SPACE.xl` and the widest shortcut, capped at `MENU_MAX_W`, or the
    /// width set with `.width`. Down: `SPACE.xs` above and below the rows,
    /// `MENU_ROW_H` per item and `SPACE.sm` per separator, and the drop of
    /// the shadow.
    pub fn size(&self) -> (u32, u32) {
        let s = scale();
        let sp = |d: u16| s.px(u32::from(d));
        let rows = self.items.iter().fold(0u32, |h, it| h.saturating_add(Self::row_h(it)));
        let h = rows.saturating_add(sp(SPACE.xs).saturating_mul(2)).saturating_add(Self::drop());
        let (label, keys) = self.widest();
        let keys = if keys == 0 { 0 } else { keys.saturating_add(sp(SPACE.xl)) };
        let edges = (sp(SPACE.xs) + sp(SPACE.md)).saturating_mul(2).saturating_add(self.lead());
        let edges = edges.saturating_add(Self::tag_room(self.widest_tag()));
        let w = edges.saturating_add(label).saturating_add(keys).min(s.px(MENU_MAX_W));
        (self.width.unwrap_or(w), h)
    }

    /// The body text size labels and shortcuts are drawn at.
    pub(super) fn px() -> f32 {
        scale().font(TextRole::Body.px())
    }

    /// How far the shadow drops below the panel: two hairlines.
    pub(super) fn drop() -> u32 {
        scale().hair().saturating_mul(2)
    }

    /// How tall `item`'s row is: `SPACE.sm` for a separator, else `MENU_ROW_H`.
    pub(super) fn row_h(item: &Item) -> u32 {
        scale().px(if item.separator { u32::from(SPACE.sm) } else { MENU_ROW_H })
    }

    /// The room at the left of every row for a tick, `ICON` and `SPACE.sm`
    /// after it, when any item is checked or unchecked; else none.
    pub(super) fn lead(&self) -> u32 {
        let s = scale();
        let room = s.px(ICON).saturating_add(s.px(u32::from(SPACE.sm)));
        self.items.iter().find(|it| it.check.is_some()).map_or(0, |_| room)
    }

    /// The room at the left of every label for tags as wide as `w`, the
    /// widest: `w` and `SPACE.md` after it, or none when `w` is 0.
    pub(super) fn tag_room(w: u32) -> u32 {
        if w == 0 {
            0
        } else {
            w.saturating_add(scale().px(u32::from(SPACE.md)))
        }
    }

    /// The widest tag, over the items that are not separators.
    pub(super) fn widest_tag(&self) -> u32 {
        let rows = self.items.iter().filter(|it| !it.separator);
        rows.map(|it| text::width(it.tag, Self::px())).max().unwrap_or(0)
    }

    /// The widest label, with its item's indent, and the widest shortcut,
    /// over the items that are not separators.
    pub(super) fn widest(&self) -> (u32, u32) {
        let rows = self.items.iter().filter(|it| !it.separator);
        let w = |t: &str| text::width(t, Self::px());
        let label = |it: &Item| w(it.label).saturating_add(Self::indent(it));
        rows.fold((0, 0), |(l, k), it| (l.max(label(it)), k.max(w(it.shortcut))))
    }

    /// How far `item`'s tag and label sit in: `SPACE.sm` after `.indent()`,
    /// else none.
    pub(super) fn indent(item: &Item) -> u32 {
        if item.indent {
            scale().px(u32::from(SPACE.sm))
        } else {
            0
        }
    }
}
