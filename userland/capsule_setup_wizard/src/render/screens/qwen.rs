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
 * The Qwen step: which pinned tier the Terminal's qwen runs when no tier
 * is named, chosen among those that fit this machine's memory.
 */

use crate::render;
use crate::render::theme::{FG, HINT};
use crate::render::widgets::{lines, text::cat, text::size};
use crate::server::step::{default_key, list_nav, Outcome};
use crate::state::Context;

use super::qwen_rows;

const WHERE: &[&[u8]] = &[
    b"qwen runs the tier chosen here when no tier is named. Its file",
    b"comes from the NONOS model repository or a disk you import,",
    b"and is checked against its signed SHA-256 pin before it runs.",
    b"Install mode keeps it with the other answers; amnesic does not.",
];

pub fn draw(ctx: &Context) {
    render::frame(ctx, b"Qwen model", b"j/k to choose a tier", b"ENTER NEXT  ESC BACK");
    let spx = ctx.stride as usize / 4;
    let (w, h) = (ctx.width, ctx.height);
    let (buf, x) = (render::buffer(ctx), render::content_x(w));
    let y = qwen_rows::draw(buf, spx, w, h, x, 110, &ctx.qwen);
    let (mut line, mut s) = ([0u8; 96], [0u8; 24]);
    let memory: &[u8] = match ctx.qwen.memory {
        Some(m) => cat(
            &mut line,
            &[b"Memory: ", size(m, &mut s), b". A tier fits when its file size x 1.2"],
        ),
        None => b"This machine's memory could not be read, so no tier fits.",
    };
    let rule: &[&[u8]] = &[memory, b"+ 512 MiB is at most that, less 1 GiB for the system."];
    let rule = if ctx.qwen.memory.is_some() { rule } else { &rule[..1] };
    let y = lines::text(buf, spx, w, h, x, y + 10, rule, HINT);
    lines::text(buf, spx, w, h, x, y + 10, WHERE, FG);
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    if let Some(o) = list_nav(&mut ctx.qwen.sel, ctx.qwen.fit + 1, code) {
        return o;
    }
    default_key(code)
}
