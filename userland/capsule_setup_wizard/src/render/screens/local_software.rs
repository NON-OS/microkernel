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

//! The one place a person lets this machine run software it installs.

use crate::render::{self, widgets::rows};
use crate::server::step::{default_key, list_nav, Outcome};
use crate::state::Context;

const MODES: &[&[u8]] =
    &[b"Only software that ships with NONOS", b"Also programs this machine installs and proves"];

pub fn draw(ctx: &Context) {
    render::frame(
        ctx,
        b"Installed software",
        b"Programs the store installs are proved by this machine. Allow them to run?",
        b"ENTER NEXT  ESC BACK",
    );
    let spx = ctx.stride as usize / 4;
    let (w, h) = (ctx.width, ctx.height);
    let buf = render::buffer(ctx);
    rows::list(buf, spx, w, h, render::content_x(w), 110, MODES, ctx.local_sel as usize);
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    if let Some(o) = list_nav(&mut ctx.local_sel, MODES.len() as u8, code) {
        return o;
    }
    default_key(code)
}
