// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! A press while a prompt is up belongs to it: the button that acts answers
//! Act, and anywhere else, the other button included, answers Leave. Either
//! way the press goes no further. The button is the one the prompt's
//! `Dialog` draws under the point (render/prompts.rs).

use super::dialog_key::answer;
use crate::state::dialog_keys::Choice;
use crate::state::Context;

/// Whether a prompt was up and took the press at `(px, py)`.
pub fn click(ctx: &mut Context, px: u32, py: u32) -> bool {
    let at = (i32::try_from(px).unwrap_or(i32::MAX), i32::try_from(py).unwrap_or(i32::MAX));
    let Some((which, button)) = crate::render::prompts::hit(ctx, at.0, at.1) else {
        return false;
    };
    answer(ctx, which, if button == Some(0) { Choice::Act } else { Choice::Leave });
    true
}
