// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The words of the third-party-app consent prompt: a title, the app's name
//! with a permissions note, and Approve / Cancel. The shared `Dialog` cuts
//! any line too wide for the panel with `…`, so nothing the user must read
//! before granting a capability silently runs past the panel edge.

use alloc::format;

use super::prompt_buttons::APPROVE;
use super::prompts::Words;
use crate::render::ui_font::valid_str;
use crate::state::Context;

const TITLE: &str = "Launch third-party app?";
const NOTE: &str = "Its signature is checked as it loads; it gets only its manifest's permissions.";

pub(super) fn words(ctx: &Context) -> Option<Words> {
    let name = valid_str(ctx.pending_consent.as_deref()?);
    Some(Words {
        title: TITLE.into(),
        body: format!("{name}: {NOTE}"),
        buttons: APPROVE,
        warn_cut: None,
    })
}
