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

//! Draw the titles across the menu bar and, when one is open, its floating
//! drop-down over everything the desktop already painted.

use super::items::{title, TITLE_COUNT};
use super::menu::{at, items};
use super::metrics::{inset, pad_x, title_w, title_x};
use crate::render::layout::menubar_height;
use crate::render::menu_view::draw;
use crate::render::palette;
use crate::render::panel::round_fill;
use crate::render::surface::surface;
use crate::render::text_aa::{text_aa, text_aa_bold};
use crate::render::ui_font::{top_y_centered, MENU_PX};
use crate::state::Context;
use nonos_toolkit::widgets::{Menu, Rect};

pub fn paint_titles(ctx: &Context) {
    let bar_h = menubar_height();
    for i in 0..TITLE_COUNT {
        let x = title_x(ctx, i);
        let w = title_w(ctx, i);
        if x + w > ctx.width {
            break;
        }
        let open = ctx.menubar.open == Some(i);
        if open {
            let box_h = bar_h.saturating_sub(inset() * 2);
            let r = Rect { x, y: inset(), w, h: box_h };
            round_fill(ctx, r, palette::R_TILE, ctx.palette.accent_dim);
        }
        let lit = open || i == 0;
        let fg = if lit { ctx.palette.text } else { ctx.palette.text_dim };
        let top = top_y_centered(0, bar_h, MENU_PX);
        if i == 0 {
            text_aa_bold(ctx, x + pad_x(), top, title(ctx, i), fg, MENU_PX);
        } else {
            text_aa(ctx, x + pad_x(), top, title(ctx, i), fg, MENU_PX);
        }
    }
}

pub fn paint(ctx: &Context) {
    let Some(index) = ctx.menubar.open else {
        return;
    };
    let rows = items(ctx, index, true);
    draw(&mut surface(ctx), Menu::new(&rows), at(ctx, index, &rows), &ctx.menubar.pointer);
}
