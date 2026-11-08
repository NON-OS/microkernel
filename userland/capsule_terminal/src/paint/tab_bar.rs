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
use nonos_toolkit::icons::IconId;
use nonos_toolkit::widgets::{IconButton, Tabs};

use super::strip::{close, plus, tab_area, tools, TOOLS};
use super::strip_ptr::StripPointer;
use super::strip_spot::Spot;

/// The strip sits in the titlebar, so it takes the OS theme's roles whichever
/// profile the body is in.
pub fn draw_tab_bar(names: &[&str], active: usize, sp: &StripPointer, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width, fb.height);
    let spot = sp.spot(names, active);
    let hovered = match spot {
        Some(Spot::Tab(i) | Spot::Close(i)) => Some(i),
        _ => None,
    };
    let strip = Tabs::new(names, active).hovered(hovered);
    strip.draw(fb, tab_area(w, h));
    if names.len() > 1 {
        draw_closes(fb, &strip, (w, h), names.len(), sp);
    }
    let p = plus(&strip, w, h, names.len());
    IconButton::new(IconId::Plus).state(sp.look(p)).draw(fb, p);
    for (r, icon) in tools(w, h).into_iter().zip(TOOLS) {
        IconButton::new(icon).state(sp.look(r)).draw(fb, r);
    }
}

fn draw_closes(
    fb: &mut PaintBuffer,
    strip: &Tabs,
    (w, h): (u32, u32),
    n: usize,
    sp: &StripPointer,
) {
    for i in 0..n {
        let Some(mark) = strip.tab(tab_area(w, h), i).and_then(close) else { continue };
        IconButton::new(IconId::Close).state(sp.look(mark)).draw(fb, mark);
    }
}
