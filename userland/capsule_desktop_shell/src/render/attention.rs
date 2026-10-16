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

//! The system's prompt and the hidden screen, drawn last in the chrome so
//! nothing of the shell's covers them; the chrome is then in the trusted
//! band, so nothing of any window's does either.

use super::consent::{
    border, button, fill, line, margin, wrap_at, BORDER, CANCEL_BG, DIM, FG, PANEL,
};
use crate::render::layout::Rect;
use crate::render::ui_font::{line_h, TITLE_PX, UI_PX};
use crate::server::handlers::attention::{back_rect, hide_rect, panel_rect};
use crate::state::dialog_keys::Choice;
use crate::state::Context;

const HIDE_BG: u32 = 0xFF1E_4A7A;
const CURTAIN: u32 = 0xFF07_0B10;
const NOTE: &str = "Only the system shows this: no window can draw over it or answer it.";

pub fn paint(ctx: &Context) {
    if ctx.trusted.attention.hidden() {
        return paint_hidden(ctx);
    }
    if !ctx.trusted.attention.prompt() {
        return;
    }
    let p = panel_rect(ctx.width, ctx.height);
    fill(ctx, p, PANEL);
    border(ctx, p, BORDER);
    let (x, max_w) = (p.x + margin(), p.width.saturating_sub(margin() * 2));
    let mut y = p.y + margin();
    line(ctx, x, y, "Secure attention", FG, TITLE_PX, max_w);
    y += line_h(TITLE_PX) + line_h(UI_PX) / 3;
    let buttons = hide_rect(ctx.width, ctx.height);
    let mut rest = NOTE;
    while !rest.is_empty() && y + line_h(UI_PX) <= buttons.y {
        let end = wrap_at(rest, UI_PX, max_w);
        line(ctx, x, y, &rest[..end], DIM, UI_PX, max_w);
        rest = rest[end..].trim_start();
        y += line_h(UI_PX);
    }
    let act = ctx.dialog_focus.focused() == Choice::Act;
    button(ctx, buttons, HIDE_BG, "Hide screen", act);
    button(ctx, back_rect(ctx.width, ctx.height), CANCEL_BG, "Back", !act);
}

/// The whole screen covered, and the one way back said plainly.
fn paint_hidden(ctx: &Context) {
    fill(ctx, Rect { x: 0, y: 0, width: ctx.width, height: ctx.height }, CURTAIN);
    let (w, mid) = (ctx.width.saturating_sub(margin() * 2), ctx.height / 2);
    line(ctx, margin(), mid.saturating_sub(line_h(TITLE_PX)), "Screen hidden", FG, TITLE_PX, w);
    let note = "Press Ctrl+Alt+Delete to show it.";
    line(ctx, margin(), mid + line_h(UI_PX) / 2, note, DIM, UI_PX, w);
}
