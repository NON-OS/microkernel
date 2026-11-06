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

//! An open title's drop-down as the shared `Menu`, and where it opens. The
//! painter and the hit test both build it here, so a click lands on the row
//! it was drawn under.

use alloc::vec::Vec;

use super::items::rows;
use super::metrics::{title_w, title_x};
use super::off_rows::off;
use crate::render::layout::menubar_height;
use crate::render::menu_view::bounds;
use crate::state::Context;
use nonos_toolkit::widgets::{Item, Menu, Rect};

/// Title `index`'s rows, in the order the dispatcher switches on. With `dim`,
/// a row whose app is off is drawn disabled; without it, it is still hit, so
/// a press on it says why it opens nothing. Both give the same geometry.
pub(super) fn items(ctx: &Context, index: usize, dim: bool) -> Vec<Item<'static>> {
    let item = |(row, &label): (usize, &&'static str)| {
        if dim && off(index, row) {
            Item::new(label).disabled()
        } else {
            Item::new(label)
        }
    };
    rows(ctx, index).iter().enumerate().map(item).collect()
}

/// The rect the drop-down of `items` is drawn at: right under its title,
/// kept on the screen.
pub(super) fn at(ctx: &Context, index: usize, items: &[Item]) -> Rect {
    let (x, w, h) = (title_x(ctx, index), title_w(ctx, index), menubar_height());
    Menu::new(items).place(Rect { x, y: 0, w, h }, bounds(ctx.width, ctx.height))
}
