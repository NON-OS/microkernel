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

//! An open panel as a shared card over the shell: its title and note, then a
//! row per entry. In a panel that picks, every row shown is a control.

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::{CARD_RADIUS, SPACE};
use nonos_toolkit::design::text_role::TextRole;
use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{draw, text, Card, Controls, Rect, Row};

/// One frame's panel: its head, a label and value per row, and whether a
/// press on a row picks it.
pub struct PanelView<'a> {
    pub title: &'a str,
    pub note: Option<&'a str>,
    pub rows: &'a [(&'a str, &'a str)],
    pub picks: bool,
}

impl PanelView<'_> {
    fn card(&self) -> Card<'_> {
        let card = Card::new().title(self.title);
        self.note.map_or(card, |n| card.note(n))
    }

    /// The card's box: as wide as its widest line, as tall as its rows, cut
    /// to the window and centred in it, never above `top`.
    pub fn rect(&self, win_w: u32, win_h: u32, top: u32) -> Rect {
        let (px, sp) = (scale().font(TextRole::Body.px()), |d: u16| scale().px(u32::from(d)));
        let line = |(l, v): &(&str, &str)| text::width(l, px) + text::width(v, px) + sp(SPACE.xl);
        let head = text::width(self.title, px).max(text::width(self.note.unwrap_or(""), px));
        let widest = self.rows.iter().map(line).fold(head, u32::max);
        let w = (widest + sp(SPACE.lg) * 2 + sp(SPACE.xl)).min(win_w);
        let rows = Row::new("").size().1 * self.rows.len() as u32;
        let h = (self.card().size().1 + rows + sp(SPACE.lg)).min(win_h.saturating_sub(top));
        let y = (win_h.saturating_sub(h) / 2).max(top);
        Rect { x: win_w.saturating_sub(w) / 2, y, w, h }
    }

    /// Each row that fits in the card at `at`, top to bottom: its index and box.
    pub fn rows_at(&self, at: Rect) -> impl Iterator<Item = (usize, Rect)> {
        let (content, rh) = (self.card().content(at), Row::new("").size().1.max(1));
        let fit = self.rows.len().min((content.h / rh) as usize);
        (0..fit).map(move |i| (i, Rect { y: content.y + rh * i as u32, h: rh, ..content }))
    }

    /// Lays the rows out in `c` when the panel picks, then paints the shadow,
    /// the card and each row in the state `c` gives it.
    pub fn paint(&self, fb: &mut PaintBuffer, at: Rect, c: &mut Controls<usize>) {
        c.set(self.rows_at(at).filter(|_| self.picks).map(|(i, r)| (r, i)));
        let (radius, drop) = (scale().px(CARD_RADIUS), scale().hair() * 2);
        draw::round(fb, Rect { y: at.y + drop, ..at }, radius, roles().shadow);
        self.card().draw(fb, at);
        for (i, r) in self.rows_at(at) {
            let (label, value) = self.rows[i];
            Row::new(label).value(value).state(c.state(i)).draw(fb, r);
        }
    }
}
