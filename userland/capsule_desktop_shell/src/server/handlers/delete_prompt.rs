// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! A click while a desktop Delete is asking. The Delete button removes the
//! item; anywhere else, Cancel included, keeps it. Either way the click goes
//! no further, as with the other modal prompts. The panel sits where the live
//! session prompt does, so the two share one geometry.

use crate::render::layout::Rect;
use crate::server::desktop;
use crate::server::repaint::repaint;
use crate::state::dialog_keys::Choice;
use crate::state::Context;

pub(crate) use super::live_prompt::{
    install_rect as delete_rect, later_rect as cancel_rect, panel_rect,
};

pub fn click(ctx: &mut Context, px: u32, py: u32) -> bool {
    if !ctx.pending_delete.showing() {
        return false;
    }
    let confirm = hit(delete_rect(ctx.width, ctx.height), px, py);
    answer(ctx, if confirm { Choice::Act } else { Choice::Leave });
    true
}

/// Delete removes the item; Cancel, Esc or a press outside keeps it.
pub fn answer(ctx: &mut Context, choice: Choice) {
    ctx.dialog_focus.reset();
    if let Some(target) = ctx.pending_delete.answer(choice == Choice::Act) {
        desktop::delete_entry(ctx, &target.name, target.is_dir);
    }
    repaint(ctx);
}

fn hit(r: Rect, px: u32, py: u32) -> bool {
    px >= r.x && px < r.x + r.width && py >= r.y && py < r.y + r.height
}
