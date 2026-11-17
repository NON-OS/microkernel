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

//! Which menu row, if any, sits under a point, and which the pointer lights.

use super::items::placed;
use crate::render::menu_view;
use crate::state::Context;

pub fn hit(ctx: &Context, px: u32, py: u32) -> Option<usize> {
    let (menu, at) = placed(ctx)?;
    menu.hit(at, px as i32, py as i32)
}

pub fn lit(ctx: &Context) -> (Option<usize>, Option<usize>) {
    placed(ctx).map_or((None, None), |(menu, at)| menu_view::lit(&menu, at, &ctx.menu_pointer))
}
