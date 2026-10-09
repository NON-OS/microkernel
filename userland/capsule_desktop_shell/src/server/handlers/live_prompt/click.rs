// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! A click while the live session prompt is up. Install opens the installer;
//! anywhere else dismisses the prompt for this session. Either way the click
//! goes no further, as with the other modal prompts.

use super::geometry::{hit, install_rect};
use crate::server::repaint::repaint;
use crate::state::dialog_keys::Choice;
use crate::state::live_prompt::LivePrompt;
use crate::state::Context;

const INSTALLER: &[u8] = b"app.install";

pub fn click(ctx: &mut Context, px: u32, py: u32) -> bool {
    if !ctx.live_prompt.showing() {
        return false;
    }
    let choice =
        if hit(install_rect(ctx.width, ctx.height), px, py) { Choice::Act } else { Choice::Leave };
    answer(ctx, choice);
    true
}

/// Install NONOS opens the installer; Not now, Esc or a press outside the
/// panel puts the offer away for this session.
pub fn answer(ctx: &mut Context, choice: Choice) {
    ctx.live_prompt = LivePrompt::Done;
    ctx.dialog_focus.reset();
    if choice == Choice::Act {
        crate::apps_off::open(ctx, INSTALLER);
    }
    repaint(ctx);
}
