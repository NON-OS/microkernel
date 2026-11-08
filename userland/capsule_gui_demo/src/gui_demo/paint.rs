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

use alloc::format;
use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::SPACE;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::rect::{inset, split_left};
use nonos_toolkit::widgets::{Button, Checkbox, Column, Ink, Kind, Label, Rect, Switch};

use super::action::Action;
use super::app::GuiDemo;

pub fn paint(fb: &mut PaintBuffer, d: &mut GuiDemo) {
    fb.clear(roles().bg.argb());
    let s = scale();
    let gap = s.px(SPACE.md.into());
    let whole = Rect { x: 0, y: 0, w: fb.width, h: fb.height };
    let mut col = Column::new(inset(whole, s.px(SPACE.lg.into()))).gap(gap);
    let c = &mut d.controls;
    c.clear();

    let count = Button::new("Count").kind(Kind::Primary);
    let reset = Button::new("Reset");
    let (cw, bh) = count.size();
    let (at, rest) = split_left(col.next(bh), cw);
    count.state(c.place(at, Action::Count)).draw(fb, at);
    let at = Rect { w: reset.size().0, ..split_left(rest, gap).1 };
    let look = match d.clicks {
        0 => c.place_disabled(at, Action::Reset),
        _ => c.place(at, Action::Reset),
    };
    reset.state(look).draw(fb, at);

    let switch = Switch::new(d.on);
    let (sw, sh) = switch.size();
    let (at, rest) = split_left(col.next(sh), sw);
    switch.state(c.place(at, Action::Toggle)).draw(fb, at);
    Label::new("Switch").draw(fb, split_left(rest, gap).1);

    let check = Checkbox::new(d.remember).label("Remember");
    let (kw, kh) = check.size();
    let at = Rect { w: kw, ..col.next(kh) };
    check.state(c.place(at, Action::Remember)).draw(fb, at);

    let status = match d.last {
        None => format!("Clicks: {}", d.clicks),
        Some(a) => format!("Last: {}. Clicks: {}", a.name(), d.clicks),
    };
    let line = Label::new(&status).ink(Ink::Muted);
    let at = col.next(line.size().1);
    line.draw(fb, at);
}
