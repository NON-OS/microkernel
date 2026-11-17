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

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::size::{CONTROL_H, SPACE};
use nonos_toolkit::icons::IconId;
use nonos_toolkit::widgets::{Card, Controls, IconButton, Ink, Kind, Label, Rect as Area};
use nonos_toolkit::widgets::{Row, Separator, State};

use super::cells::{back, below, px};
use crate::browser::ui::list_geom::ListGeom;
use crate::browser::ui::page_part::PagePart;
use crate::browser::ui::rect::{area, Rect};

/// The card behind `n` rows and the hairlines between them.
pub fn frame(fb: &mut PaintBuffer, g: &ListGeom, n: usize) {
    Card::new().draw(fb, area(g.card(n)));
    let (pad, hair) = (px(u32::from(SPACE.lg)), Separator::new().size().1 as i32);
    for i in 1..n {
        let y = g.row_rect(i).y;
        if y >= g.band && y < fb.height as i32 {
            let r = Rect::new(g.frame.x + pad, y - hair / 2, g.frame.w - 2 * pad, hair);
            Separator::new().soft().draw(fb, area(r));
        }
    }
}

/// True when some of `row` shows below the header band. A row wholly
/// under the band is not drawn, so a drawn row never starts above the top.
pub fn shown(fb: &PaintBuffer, g: &ListGeom, row: Rect) -> bool {
    row.bottom() > g.band && row.y >= 0 && row.y < fb.height as i32
}

/// Draws `row` in `r`, a control opening `open` when it has one, with a
/// Close icon button at the right of its trailing box removing `remove`
/// when it has one, laid over the row. Each takes clicks only below the
/// band, and each is lit only where a click on it lands. Returns the row's
/// leading and trailing boxes.
pub fn place(
    fb: &mut PaintBuffer,
    g: &ListGeom,
    c: &mut Controls<PagePart>,
    (r, row): (Rect, Row),
    (open, remove): (Option<PagePart>, Option<PagePart>),
) -> (Area, Area) {
    let at = area(r);
    let t = back(row.trailing_rect(at));
    let side = px(CONTROL_H).min(t.h);
    let b = Rect::new(t.right() - side, t.y + (t.h - side) / 2, side, side);
    if let Some(o) = open {
        c.push(area(below(r, g.band)), o);
    }
    if let Some(rm) = remove {
        c.push(area(below(b, g.band)), rm);
    }
    let row = row.state(open.map_or(State::default(), |o| c.state(o)));
    row.draw(fb, at);
    if let Some(rm) = remove {
        let st = c.state(rm);
        IconButton::new(IconId::Close).kind(Kind::Plain).state(st).draw(fb, area(b));
    }
    (row.leading_rect(at), area(t))
}

/// A one-row card holding `msg`, for a list with nothing in it.
pub fn empty(fb: &mut PaintBuffer, g: &ListGeom, msg: &str) {
    frame(fb, g, 1);
    let (r, pad) = (g.row_rect(0), px(u32::from(SPACE.lg)));
    if shown(fb, g, r) {
        let at = area(Rect::new(r.x + pad, r.y, r.w - 2 * pad, r.h));
        Label::new(msg).ink(Ink::Muted).draw(fb, at);
    }
}
