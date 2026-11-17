// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The answer to the live session prompt. Install opens the installer;
//! anything else dismisses the prompt for this session.

use crate::server::repaint::repaint;
use crate::state::dialog_keys::Choice;
use crate::state::live_prompt::LivePrompt;
use crate::state::Context;

const INSTALLER: &[u8] = b"app.install";

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
