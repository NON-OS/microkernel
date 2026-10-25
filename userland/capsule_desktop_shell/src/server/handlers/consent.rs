// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The consent gate for a runtime-installed third-party app: clicking its
//! Launchpad tile raises a modal instead of launching. Approve runs the app;
//! any other click, Cancel included, dismisses it. Geometry lives here as the
//! single source of truth the paint pass hit-tests against.

use super::consent_rect::approve_rect;
use crate::render::layout::Rect;
use crate::server::repaint::repaint;
use crate::state::dialog_keys::Choice;
use crate::state::Context;

pub(crate) fn click(ctx: &mut Context, px: u32, py: u32) -> bool {
    if ctx.pending.pending_consent.is_none() {
        return false;
    }
    let approve = hit(approve_rect(ctx.width, ctx.height), px, py);
    answer(ctx, if approve { Choice::Act } else { Choice::Leave });
    true
}

/// Approve launches the app; Cancel, Esc or a press outside does not.
pub(crate) fn answer(ctx: &mut Context, choice: Choice) {
    ctx.dialog_focus.reset();
    if let Some(name) = ctx.pending.pending_consent.take() {
        if choice == Choice::Act {
            super::installed_launch::launch(ctx, &name);
        }
    }
    repaint(ctx);
}

fn hit(r: Rect, px: u32, py: u32) -> bool {
    px >= r.x && px < r.x + r.width && py >= r.y && py < r.y + r.height
}
