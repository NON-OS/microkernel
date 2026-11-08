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

//! The explorer's rows: a navigation item per visible tree node, folder or
//! file icon, the selected node active, indented by depth with a chevron
//! before a directory. The whole row is its control, chevron included.

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::{ICON, SPACE};
use nonos_toolkit::icons::IconId;
use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::rect::split_left;
use nonos_toolkit::widgets::{draw, Controls, NavItem, Rect};

/// One visible tree node, as the explorer draws it.
#[derive(Clone, Copy)]
pub struct Entry<'a> {
    pub name: &'a str,
    pub depth: u16,
    pub dir: bool,
    pub open: bool,
}

/// One frame's explorer: the visible nodes, the first one scrolled into view,
/// the selected one, the band the rows fill and each row's height.
pub struct Explorer<'a> {
    pub entries: &'a [Entry<'a>],
    pub first: usize,
    pub selected: usize,
    pub band: Rect,
    pub row_h: u32,
}

impl Explorer<'_> {
    /// Each row in view, top to bottom, while it fits: its index and box.
    pub fn rows(&self) -> impl Iterator<Item = (usize, Rect)> + '_ {
        let fit = (self.band.h / self.row_h.max(1)) as usize;
        let shown = (self.first..self.entries.len()).take(fit).enumerate();
        let at = |r: usize| self.band.y.saturating_add(self.row_h.saturating_mul(r as u32));
        shown.map(move |(r, vi)| (vi, Rect { y: at(r), h: self.row_h, ..self.band }))
    }

    /// Lays every row out in `c`, then paints each in the state `c` gives it.
    pub fn paint(&self, fb: &mut PaintBuffer, c: &mut Controls<usize>) {
        c.set(self.rows().map(|(vi, at)| (at, vi)));
        let step = scale().px(u32::from(SPACE.md));
        for (vi, at) in self.rows() {
            let e = self.entries[vi];
            let (gutter, body) = split_left(at, step.saturating_mul(u32::from(e.depth) + 1));
            if e.dir {
                let side = scale().px(ICON).min(at.h);
                let x = (gutter.x + gutter.w).saturating_sub((step + side) / 2);
                let id = if e.open { IconId::ChevronDown } else { IconId::ChevronRight };
                draw::icon(fb, id, x, at.y + (at.h - side) / 2, side, roles().muted);
            }
            let icon = if e.dir { IconId::FsFolder } else { IconId::FsFile };
            let item = NavItem::new(e.name).icon(icon).active(vi == self.selected);
            item.state(c.state(vi)).draw(fb, body);
        }
    }
}
