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

//! Painter for the table-driven sections: the section's card, then a row per
//! setting, each laid out in the screen's controls as it is drawn. The whole
//! row is the control, so it lights exactly where a press on it acts.

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::widgets::{Card, Controls, Rect, Row, Separator, State, Switch};

use super::card::{card_rect, row_rect};
use super::pick::Pick;
use super::sect::{Ctl, Section};
use super::sect_state::sect_on;
use crate::editor::theme;

pub(super) fn paint_section(
    fb: &mut PaintBuffer,
    nav: usize,
    sec: &Section,
    c: &mut Controls<Pick>,
) {
    let card = Card::new().title(sec.head);
    let at = card_rect(fb.width, sec.head, sec.rows.len());
    card.draw(fb, at);
    let content = card.content(at);
    let hair = scale().hair();
    for (i, spec) in sec.rows.iter().enumerate() {
        let row = row_rect(content, i);
        paint_row(fb, row, nav, spec, c.place(row, Pick::Row(i)));
        if i > 0 {
            let line =
                Rect { x: row.x + hair, y: row.y, w: row.w.saturating_sub(hair * 2), h: hair };
            Separator::new().soft().draw(fb, line);
        }
    }
}

fn paint_row(fb: &mut PaintBuffer, at: Rect, nav: usize, spec: &(&str, Ctl), st: State) {
    match spec.1 {
        Ctl::Toggle(bit) => {
            let switch = Switch::new(sect_on(nav, bit)).state(st);
            let row = Row::new(spec.0).trailing(switch.size().0).state(st);
            row.draw(fb, at);
            switch.draw(fb, row.trailing_rect(at));
        }
        Ctl::Theme => Row::new(spec.0).value(theme::name()).chevron().state(st).draw(fb, at),
    }
}
