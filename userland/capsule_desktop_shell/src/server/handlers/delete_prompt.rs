// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! A click while a desktop Delete is asking. The Delete button removes the
//! item; anywhere else, Cancel included, keeps it. Either way the click goes
//! no further, as with the other modal prompts (server/handlers/
//! prompt_click.rs).

use crate::server::desktop;
use crate::server::repaint::repaint;
use crate::state::dialog_keys::Choice;
use crate::state::Context;

/// Delete removes the item; Cancel, Esc or a press outside keeps it.
pub fn answer(ctx: &mut Context, choice: Choice) {
    ctx.dialog_focus.reset();
    if let Some(target) = ctx.pending_delete.answer(choice == Choice::Act) {
        desktop::delete_entry(ctx, &target.name, target.is_dir);
    }
    repaint(ctx);
}
