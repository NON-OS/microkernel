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
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::SPACE;
use nonos_toolkit::widgets::{Align, Card, Ink, Label, Rect as Area};

use crate::browser::ui::rect::Rect;

const STEP: u32 = 28;
const COUNT_W: u32 = 40;

/// One line of a start card: its text and ink, and a count shown muted at
/// the card's right.
pub type Line<'a> = (&'a str, Ink, Option<&'a str>);

/// A card titled `title` in `r` holding `lines` under its head, each cut
/// to the room left of the count column. A card scrolled past the top
/// keeps its body but not its title.
pub fn paint(fb: &mut PaintBuffer, r: Rect, title: &str, lines: &[Line]) {
    let s = scale();
    let card = if r.y >= 0 { Card::new().title(title) } else { Card::new() };
    let at = area(r);
    card.draw(fb, at);
    let (pad, step, count_w) = (s.px(u32::from(SPACE.lg)), s.px(STEP), s.px(COUNT_W));
    let (x, w) = (r.x + pad as i32, r.w - 2 * pad as i32);
    let top = r.y + Card::new().title(title).head_h() as i32;
    for (i, &(text, ink, count)) in lines.iter().enumerate() {
        let y = top + i as i32 * step as i32;
        if y < 0 {
            continue;
        }
        let line = Rect::new(x, y, w - count_w as i32, step as i32);
        Label::new(text).ink(ink).draw(fb, area(line));
        if let Some(n) = count {
            let tail = area(Rect::new(x + w - count_w as i32, y, count_w as i32, step as i32));
            Label::new(n).ink(Ink::Muted).align(Align::Right).draw(fb, tail);
        }
    }
}

/// `r` as the toolkit's rect, its negative parts cut off.
fn area(r: Rect) -> Area {
    let (x, y) = (r.x.max(0), r.y.max(0));
    let (w, h) = ((r.right() - x).max(0), (r.bottom() - y).max(0));
    Area { x: x as u32, y: y as u32, w: w as u32, h: h as u32 }
}
