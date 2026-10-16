// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Paint the third-party-app consent modal above the desktop: a centred panel
//! carrying a title, the app name, a permissions note, and Approve / Cancel
//! buttons. Button rects come from the handler so paint and hit-test agree.
//! Every width is measured from the face, so nothing the user must read before
//! granting a capability can silently run past the panel edge.

use super::dialog_kit::{
    border, button, fill, margin, APPROVE_BG, BORDER, CANCEL_BG, DIM, FG, PANEL,
};
use super::dialog_text::{line, wrap_at};
use crate::render::ui_font::{line_h, valid_str, TITLE_PX, UI_PX};
use crate::server::handlers::consent_rect::{approve_rect, cancel_rect, panel_rect};
use crate::state::dialog_keys::Choice;
use crate::state::Context;

const TITLE: &str = "Launch third-party app?";

const NOTE: &str = "Its signature is checked as it loads; it gets only its manifest's permissions.";

pub fn paint_consent(ctx: &Context) {
    let Some(name) = ctx.pending.pending_consent.as_deref() else {
        return;
    };
    let p = panel_rect(ctx.width, ctx.height);
    fill(ctx, p, PANEL);
    border(ctx, p, BORDER);
    let x = p.x + margin();
    let max_w = p.width.saturating_sub(margin() * 2);
    let mut y = p.y + margin();
    line(ctx, x, y, TITLE, FG, TITLE_PX, max_w);
    y += line_h(TITLE_PX);
    line(ctx, x, y, valid_str(name), FG, UI_PX, max_w);
    y += line_h(UI_PX);
    note(ctx, x, y, max_w);
    let act = ctx.dialog_focus.focused() == Choice::Act;
    button(ctx, approve_rect(ctx.width, ctx.height), APPROVE_BG, "Approve", act);
    button(ctx, cancel_rect(ctx.width, ctx.height), CANCEL_BG, "Cancel", !act);
}

fn note(ctx: &Context, x: u32, top_y: u32, max_w: u32) {
    let mut rest = NOTE;
    let mut y = top_y;
    while !rest.is_empty() {
        let end = wrap_at(rest, UI_PX, max_w);
        line(ctx, x, y, &rest[..end], DIM, UI_PX, max_w);
        rest = rest[end..].trim_start();
        y += line_h(UI_PX);
    }
}
