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

//! Paint the display input menu: the prompts' panel, over the toasts, in
//! the trusted band while it is up. Each row names its display and one
//! input the display lists; the keyboard's row wears the ring, and the
//! input a display shows now says so.

use super::dialog_kit::{border, button, fill, margin, BORDER, CANCEL_BG, DIM, FG, PANEL};
use super::dialog_text::line;
use crate::render::ui_font::{line_h, TITLE_PX, UI_PX};
use crate::server::handlers::input_menu_geometry::{first_row, panel_rect, row_rect, VISIBLE};
use crate::state::input_names::name;
use crate::state::Context;

const TITLE: &str = "Choose a display's input";
const NOTE: &str = "The picture moves to that input. Switch back on the display.";

pub fn paint_input_menu(ctx: &Context) {
    let menu = &ctx.trusted.input_menu;
    if !menu.showing() {
        return;
    }
    let rows = menu.rows().count();
    let p = panel_rect(ctx.width, ctx.height, rows);
    fill(ctx, p, PANEL);
    border(ctx, p, BORDER);
    let (x, max_w) = (p.x + margin(), p.width.saturating_sub(margin() * 2));
    let y = p.y + margin();
    line(ctx, x, y, TITLE, FG, TITLE_PX, max_w);
    line(ctx, x, y + line_h(TITLE_PX) + line_h(UI_PX) / 3, NOTE, DIM, UI_PX, max_w);
    let first = first_row(menu.at());
    for (i, r) in menu.rows().enumerate().skip(first).take(VISIBLE) {
        let mut text = [0u8; 40];
        let n = label(r.display, r.input, r.showing, &mut text);
        let s = core::str::from_utf8(&text[..n]).unwrap_or("?");
        let at = row_rect(ctx.width, ctx.height, rows, i - first);
        button(ctx, at, CANCEL_BG, s, i == menu.at());
    }
}

/// "Display 2: HDMI 1 (showing)", written into `out`; its length.
fn label(display: usize, input: u8, showing: bool, out: &mut [u8; 40]) -> usize {
    let head = [b'D', b'i', b's', b'p', b'l', b'a', b'y', b' ', b'1' + display as u8, b':', b' '];
    out[..head.len()].copy_from_slice(&head);
    let mut named = [0u8; 16];
    let k = name(input, &mut named);
    out[head.len()..head.len() + k].copy_from_slice(&named[..k]);
    let mut n = head.len() + k;
    if showing {
        out[n..n + 10].copy_from_slice(b" (showing)");
        n += 10;
    }
    n
}
