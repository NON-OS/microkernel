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

/*
 * The name step on screen: the name as typed, the rules, and why the last
 * refused key was refused.
 */

use crate::name::{check, NAME_MAX};
use crate::render;
use crate::render::theme::{FG, HINT};
use crate::render::widgets::{lines, rows, text::cat};
use crate::server::step::Outcome;
use crate::state::Context;
use crate::text::{key, Typed};

const RULES: &[&[u8]] = &[
    b"Lowercase letters, digits, - and _, starting with a letter.",
    b"1 to 32 characters. Left empty, the name is nonos.",
    b"Install mode keeps it with the other answers; amnesic does not.",
];

pub fn draw(ctx: &Context) {
    render::frame(
        ctx,
        b"Your name",
        b"The account name the Terminal shows as name@host.",
        b"TYPE A NAME  ENTER NEXT  ESC BACK",
    );
    let spx = ctx.stride as usize / 4;
    let (w, h) = (ctx.width, ctx.height);
    let l = render::layout_of(ctx);
    let (buf, x) = (render::buffer(ctx), l.col_x);
    let n = &ctx.name;
    let mut shown = [0u8; NAME_MAX + 1];
    let typed = cat(&mut shown, &[n.typed(), b"_"]);
    let y = rows::list(buf, spx, w, h, x, l.body_y, &[typed], 0);
    let y = lines::text(buf, spx, w, h, x, y + l.gap, RULES, HINT);
    if let Some((byte, why)) = n.refused {
        let mut line = [0u8; 64];
        let said = cat(&mut line, &[b"Refused \"", &[byte], b"\": ", why.text()]);
        lines::text(buf, spx, w, h, x, y + l.gap, &[said], FG);
    }
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    let n = &mut ctx.name;
    match key(&mut n.line, &mut n.len, code, check) {
        Typed::Changed => n.refused = None,
        Typed::Refused(why) => n.refused = Some((code as u8, why)),
        Typed::Enter => return Outcome::Advance,
        Typed::Esc => return Outcome::Back,
        Typed::Ignored => {}
    }
    Outcome::Stay
}
