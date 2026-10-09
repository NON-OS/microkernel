// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The consent gate for a runtime-installed third-party app: clicking its
//! Launchpad tile raises a modal instead of launching. Approve runs the app;
//! any other click, Cancel included, dismisses it. Geometry lives here as the
//! single source of truth the paint pass hit-tests against.

use crate::render::layout::Rect;
use crate::render::ui_font;
use crate::server::repaint::repaint;
use crate::state::dialog_keys::Choice;
use crate::state::Context;

const PANEL_W_LOGICAL: u32 = 560;
const PANEL_H_LOGICAL: u32 = 170;
const BTN_W_LOGICAL: u32 = 110;
const BTN_H_LOGICAL: u32 = 32;

fn panel_w() -> u32 {
    ui_font::px(PANEL_W_LOGICAL)
}

fn panel_h() -> u32 {
    ui_font::px(PANEL_H_LOGICAL)
}

fn btn_w() -> u32 {
    ui_font::px(BTN_W_LOGICAL)
}

fn btn_h() -> u32 {
    ui_font::px(BTN_H_LOGICAL)
}

pub(crate) fn panel_rect(w: u32, h: u32) -> Rect {
    let pw = core::cmp::min(panel_w(), w);
    let ph = core::cmp::min(panel_h(), h);
    Rect { x: w.saturating_sub(pw) / 2, y: h.saturating_sub(ph) / 2, width: pw, height: ph }
}

pub(crate) fn approve_rect(w: u32, h: u32) -> Rect {
    button_rect(w, h, true)
}

pub(crate) fn cancel_rect(w: u32, h: u32) -> Rect {
    button_rect(w, h, false)
}

fn button_rect(w: u32, h: u32, approve: bool) -> Rect {
    let p = panel_rect(w, h);
    let (bw, bh) = (btn_w(), btn_h());
    let y = p.y + p.height.saturating_sub(bh + ui_font::px(16));
    let mid = p.x + p.width / 2;
    let x = if approve { mid.saturating_sub(bw + ui_font::px(8)) } else { mid + ui_font::px(8) };
    Rect { x, y, width: bw, height: bh }
}

pub(crate) fn click(ctx: &mut Context, px: u32, py: u32) -> bool {
    if ctx.pending_consent.is_none() {
        return false;
    }
    let approve = hit(approve_rect(ctx.width, ctx.height), px, py);
    answer(ctx, if approve { Choice::Act } else { Choice::Leave });
    true
}

/// Approve launches the app; Cancel, Esc or a press outside does not.
pub(crate) fn answer(ctx: &mut Context, choice: Choice) {
    ctx.dialog_focus.reset();
    if let Some(name) = ctx.pending_consent.take() {
        if choice == Choice::Act {
            super::installed_launch::launch(ctx, &name);
        }
    }
    repaint(ctx);
}

fn hit(r: Rect, px: u32, py: u32) -> bool {
    px >= r.x && px < r.x + r.width && py >= r.y && py < r.y + r.height
}
