// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! While the install prompt is up it swallows every click: Approve commits the
//! package against the digest the query returned, anything else dismisses it.

use alloc::vec::Vec;

use super::{approve_rect, geometry::hit};
use crate::render::sync_toast_layer;
use crate::server::repaint::repaint;
use crate::state::dialog_keys::Choice;
use crate::state::{Context, NotifyLevel, PkgInstallPrompt};

/// The commit blocks this single-threaded loop for as long as the installer
/// takes, so the modal is torn down and the frame handed to the compositor
/// before it starts: otherwise the stale panel sits on screen for the whole
/// install with no sign that the click registered.
pub(crate) fn click(ctx: &mut Context, px: u32, py: u32) -> bool {
    if ctx.pending_pkg_install.is_none() {
        return false;
    }
    let approve = hit(approve_rect(ctx.width, ctx.height), px, py);
    answer(ctx, if approve { Choice::Act } else { Choice::Leave });
    true
}

/// Approve installs the package; Cancel, Esc or a press outside does not.
pub(crate) fn answer(ctx: &mut Context, choice: Choice) {
    ctx.dialog_focus.reset();
    let Some(prompt) = ctx.pending_pkg_install.take() else {
        return;
    };
    if choice == Choice::Leave {
        repaint(ctx);
        return;
    }
    let mut text = Vec::with_capacity(32);
    text.extend_from_slice(b"installing ");
    text.extend_from_slice(&prompt.summary.slug);
    ctx.toasts.push(&text, NotifyLevel::Info, crate::server::toast_clock::now());
    sync_toast_layer(ctx);
    repaint(ctx);
    commit(ctx, prompt);
    repaint(ctx);
}

/// The commit re-verifies against the digest the prompt was built from, so a
/// file swapped between the query and the click fails rather than installing
/// something the user never saw.
fn commit(ctx: &mut Context, prompt: PkgInstallPrompt) {
    match crate::installer_client::pkg_commit(&prompt.path, &prompt.summary.digest) {
        Ok(()) => {
            let mut text = Vec::with_capacity(32);
            text.extend_from_slice(b"installed ");
            text.extend_from_slice(&prompt.summary.slug);
            ctx.toasts.push(&text, NotifyLevel::Info, crate::server::toast_clock::now());
            sync_toast_layer(ctx);
        }
        Err(code) => crate::server::handlers::pkg_install::report_rejected(ctx, code),
    }
}
