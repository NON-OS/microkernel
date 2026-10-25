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

//! Painting one tab: its header, body, input line and footer.

use nonos_app_skeleton::PaintBuffer;

use crate::layout::Layout;
use crate::paint::block_chrome::draw_block_chrome;
use crate::paint::draw_input_line::draw_input_line;
use crate::paint::fetch::draw_fetch;
use crate::paint::footer::draw_footer;
use crate::paint::geometry::geometry;
use crate::paint::header::draw_header;
use crate::paint::vt::{draw_find_bar, draw_vt, Area, Frame, Rows};
use crate::term::state::State;
use crate::term::theme::types::Theme;

pub fn paint(
    state: &State,
    fb: &mut PaintBuffer,
    t: &Theme,
    font_scale: u32,
    rail_open: bool,
) -> Layout {
    fb.clear(t.bg);
    draw_header(state, fb, t);
    let (l, m, chrome) = geometry(fb, font_scale, rail_open);
    let text_x = l.body.x + chrome.text_left;
    let text_r = (l.body.x + l.body.w).saturating_sub(chrome.text_left);
    let vt = &state.scrollback.vt;
    /*
     * A program in the foreground, or on the alternate screen, owns the
     * whole body and is drawn as its screen; the prompt returns at its end.
     */
    let owned = vt.alt_active() || state.fg_running;
    let bottom = if owned { l.footer.y } else { l.input.y };
    let area = Area { x: text_x, y: l.body.y, max_x: text_r, max_y: bottom };
    let f = Frame { vt, area, m, t };
    if owned {
        draw_vt(&f, fb, Rows::Screen, state.shade());
    } else if state.fresh {
        draw_fetch(fb, text_x, l.body.y, text_r, t);
    } else {
        let rows = Rows::Shell { rows: (l.body.h / m.lh.max(1)) as usize, back: vt.view_offset() };
        draw_block_chrome(state, fb, &f.area, rows.first(vt), &m, t);
        draw_vt(&f, fb, rows, state.shade());
    }
    if let Some(find) = &state.find {
        draw_find_bar(find, fb, l.input, m, t);
    } else if !owned {
        draw_input_line(state, fb, l.input, m, t);
    }
    draw_footer(fb, t);
    l
}
