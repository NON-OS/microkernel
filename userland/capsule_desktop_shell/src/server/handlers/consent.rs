// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The consent gate for a runtime-installed third-party app: clicking its
//! Launchpad tile raises a modal instead of launching. Approve runs the app;
//! any other click, Cancel included, dismisses it (server/handlers/
//! prompt_click.rs).

use crate::server::repaint::repaint;
use crate::state::dialog_keys::Choice;
use crate::state::Context;

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
