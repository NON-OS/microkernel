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

use nonos_app_skeleton::{measure_ttf, PaintBuffer};
use nonos_toolkit::font::ttf::line_height;
use nonos_toolkit::icons::IconId;
use nonos_toolkit::widgets::text::truncate_by;
use nonos_toolkit::widgets::{Button, Controls, IconButton, Kind, Rect as Area};

use crate::browser::paint::tokens::{self, palette};
use crate::browser::shell::kept::{ReaderPrefs, MAX_SIZE};
use crate::browser::ui::reader_geom::ReaderGeom;
use crate::browser::ui::reader_hit::Control;
use crate::browser::ui::rect::Rect;

const RADIUS: u32 = 10;
const LABEL_PX: f32 = 17.0;
const THEMES: [&str; 3] = ["Light", "Sepia", "Dark"];
const WIDTHS: [&str; 3] = ["Narrow", "Standard", "Wide"];
const DOT: i32 = 8;
const STEPS: i32 = 5;

/// The three theme swatches, each in the colours of the theme it picks.
pub fn swatches(p: &ReaderPrefs, g: &ReaderGeom, fb: &mut PaintBuffer, c: &mut Controls<Control>) {
    for (i, r) in g.swatches.iter().enumerate() {
        c.push(area(*r), Control::Theme(i as u8));
        let [bg, fg, _] = tokens::READER_THEMES[i];
        let (t, edge) =
            if p.theme as usize == i { (2, palette().accent) } else { (1, palette().edge) };
        fb.fill_round(r.x as u32, r.y as u32, r.w as u32, r.h as u32, RADIUS, bg);
        fb.stroke_round(r.x as u32, r.y as u32, r.w as u32, r.h as u32, RADIUS, t, edge);
        let s = truncate_by(THEMES[i], r.w - 8, |t| measure_ttf(t, LABEL_PX));
        let x = r.x + (r.w - measure_ttf(&s, LABEL_PX)) / 2;
        fb.text_ttf(x, r.y + (r.h - line_height(LABEL_PX)) / 2, &s, fg, LABEL_PX);
    }
}

/// The text size steps, a minus and a plus each disabled at its end, and
/// the size dots between them.
pub fn size(p: &ReaderPrefs, g: &ReaderGeom, fb: &mut PaintBuffer, c: &mut Controls<Control>) {
    let steps = [
        (g.minus, Control::Smaller, IconId::Minus, p.size > 0),
        (g.plus, Control::Larger, IconId::Plus, p.size < MAX_SIZE),
    ];
    for (r, ctl, id, live) in steps {
        let st = if live { c.place(area(r), ctl) } else { c.place_disabled(area(r), ctl) };
        IconButton::new(id).state(st).draw(fb, area(r));
    }
    let (step, cy) = (g.dots.w / (STEPS + 1), g.dots.y + g.dots.h / 2);
    for i in 0..STEPS {
        let x = g.dots.x + step * (i + 1) - DOT / 2;
        let argb = if i <= p.size as i32 { palette().accent } else { palette().edge };
        let (d, y) = (DOT as u32, (cy - DOT / 2) as u32);
        fb.fill_round(x as u32, y, d, d, d / 2, argb);
    }
}

/// The content width choices, the one in force Primary.
pub fn widths(p: &ReaderPrefs, g: &ReaderGeom, fb: &mut PaintBuffer, c: &mut Controls<Control>) {
    for (i, r) in g.widths.iter().enumerate() {
        let st = c.place(area(*r), Control::Width(i as u8));
        let kind = if p.width as usize == i { Kind::Primary } else { Kind::Secondary };
        Button::new(WIDTHS[i]).kind(kind).state(st).draw(fb, area(*r));
    }
}

/// `r` as the toolkit's rect, its negative parts cut off.
pub fn area(r: Rect) -> Area {
    let (x, y) = (r.x.max(0), r.y.max(0));
    let (w, h) = ((r.right() - x).max(0), (r.bottom() - y).max(0));
    Area { x: x as u32, y: y as u32, w: w as u32, h: h as u32 }
}
