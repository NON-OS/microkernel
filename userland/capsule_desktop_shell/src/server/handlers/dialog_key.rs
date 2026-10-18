// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! A key while a dialog or a menu is up belongs to it, in the order a press
//! is offered to them (`server/input/`): the dialog drawn on top answers
//! first, and the display input menu, a list, takes its own keys. Tab moves between a dialog's buttons, Enter presses the one with
//! the ring, Esc leaves things as they are; Esc closes an open menu.

use super::menu_key::menu_key;
use crate::server::repaint::repaint;
use crate::state::dialog_keys::Choice;
use crate::state::input_source_rule::Routed;
use crate::state::Context;

#[derive(Clone, Copy)]
enum Dialog {
    Attention,
    PowerOff,
    Live,
    Delete,
    Consent,
    Package,
}

/// Whether the key went to a dialog or a menu.
pub fn key(ctx: &mut Context, by: Routed, code: u32) -> bool {
    // The input menu is a list, not two buttons: it takes its own keys.
    if !ctx.trusted.attention.prompt() && super::input_menu::key(ctx, by, code) {
        return true;
    }
    let which = if ctx.trusted.attention.prompt() {
        Dialog::Attention
    } else if ctx.trusted.power_off.showing() {
        Dialog::PowerOff
    } else if ctx.pending.live_prompt.showing() {
        Dialog::Live
    } else if ctx.pending.pending_delete.showing() {
        Dialog::Delete
    } else if ctx.pending.pending_consent.is_some() {
        Dialog::Consent
    } else if ctx.pending.pending_pkg_install.is_some() {
        Dialog::Package
    } else {
        return menu_key(ctx, code);
    };
    match ctx.dialog_focus.key(code) {
        Some(choice) => answer(ctx, by, which, choice),
        None => repaint(ctx),
    }
    true
}

fn answer(ctx: &mut Context, by: Routed, which: Dialog, choice: Choice) {
    match which {
        Dialog::Attention => super::attention::answer(ctx, choice),
        Dialog::PowerOff => super::power_off::answer(ctx, by, choice),
        Dialog::Live => super::live_prompt::answer(ctx, choice),
        Dialog::Delete => super::delete_prompt::answer(ctx, choice),
        Dialog::Consent => super::consent::answer(ctx, choice),
        Dialog::Package => super::pkg_consent::answer(ctx, choice),
    }
}
