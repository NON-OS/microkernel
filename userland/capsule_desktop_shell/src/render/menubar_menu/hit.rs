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

//! Which title, and which row of the open drop-down, sits under a point, and
//! which row the pointer lights.

use super::items::TITLE_COUNT;
use super::menu::{at, items};
use super::metrics::{title_w, title_x};
use crate::render::layout::menubar_height;
use crate::render::menu_view;
use crate::render::topbar::brand_right;
use crate::state::Context;
use nonos_toolkit::widgets::Menu;

pub fn title_hit(ctx: &Context, px: u32, py: u32) -> Option<usize> {
    if py >= menubar_height() || px < brand_right() {
        return None;
    }
    (0..TITLE_COUNT).find(|&i| {
        let x = title_x(ctx, i);
        px >= x && px < x + title_w(ctx, i)
    })
}

pub fn row_hit(ctx: &Context, px: u32, py: u32) -> Option<usize> {
    let index = ctx.menubar.open?;
    let rows = items(ctx, index, false);
    Menu::new(&rows).hit(at(ctx, index, &rows), px as i32, py as i32)
}

pub fn lit(ctx: &Context) -> (Option<usize>, Option<usize>) {
    let Some(index) = ctx.menubar.open else {
        return (None, None);
    };
    let rows = items(ctx, index, true);
    menu_view::lit(&Menu::new(&rows), at(ctx, index, &rows), &ctx.menubar.pointer)
}
