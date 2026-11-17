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

//! Echoing keys in the theme chosen in Settings.

use nonos_app_skeleton::runner::follow_theme;
use nonos_toolkit::theme::roles;

use super::draw::draw;
use crate::state::Context;

pub fn push_and_draw(ctx: &mut Context, ascii: u8) {
    if ctx.cursor >= ctx.buf.len() {
        ctx.cursor = 0;
    }
    ctx.buf[ctx.cursor] = ascii;
    ctx.cursor += 1;
    redraw(ctx);
}

/// The surface as it stands, in the theme in force.
pub fn redraw(ctx: &Context) {
    let _ = follow_theme();
    draw(ctx, &roles());
}
