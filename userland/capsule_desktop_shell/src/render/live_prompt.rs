// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Paint the offer to install NONOS at the start of a live session: a centred
//! panel saying that nothing here is kept, with Install NONOS and Not now.

use super::consent::{
    border, button, fill, line, margin, wrap_at, APPROVE_BG, BORDER, CANCEL_BG, DIM, FG, PANEL,
};
use crate::render::ui_font::{line_h, TITLE_PX, UI_PX};
use crate::server::handlers::live_prompt::{install_rect, later_rect, panel_rect};
use crate::state::dialog_keys::Choice;
use crate::state::Context;

const TITLE: &str = "This is a live session";
const NOTE: &str = "Nothing here is kept: files, settings and models are gone when the machine stops. Install NONOS to keep them.";

pub fn paint_live_prompt(ctx: &Context) {
    if !ctx.live_prompt.showing() {
        return;
    }
    let p = panel_rect(ctx.width, ctx.height);
    fill(ctx, p, PANEL);
    border(ctx, p, BORDER);
    let x = p.x + margin();
    let max_w = p.width.saturating_sub(margin() * 2);
    let mut y = p.y + margin();
    line(ctx, x, y, TITLE, FG, TITLE_PX, max_w);
    y += line_h(TITLE_PX) + line_h(UI_PX) / 3;
    let buttons = install_rect(ctx.width, ctx.height);
    let mut rest = NOTE;
    while !rest.is_empty() && y + line_h(UI_PX) <= buttons.y {
        let end = wrap_at(rest, UI_PX, max_w);
        line(ctx, x, y, &rest[..end], DIM, UI_PX, max_w);
        rest = rest[end..].trim_start();
        y += line_h(UI_PX);
    }
    let act = ctx.dialog_focus.focused() == Choice::Act;
    button(ctx, buttons, APPROVE_BG, "Install NONOS", act);
    button(ctx, later_rect(ctx.width, ctx.height), CANCEL_BG, "Not now", !act);
}
