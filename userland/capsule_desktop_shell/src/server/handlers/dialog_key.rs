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
//! is offered to them (server/input.rs): the dialog drawn on top answers
//! first. Tab moves between a dialog's buttons, Enter presses the one with
//! the ring, Esc leaves things as they are; Esc closes an open menu.

use crate::server::repaint::repaint;
use crate::state::dialog_keys::{Choice, KEY_ESC};
use crate::state::Context;

#[derive(Clone, Copy)]
enum Dialog {
    Live,
    Delete,
    Consent,
    Package,
}

/// Whether the key went to a dialog or a menu.
pub fn key(ctx: &mut Context, code: u32) -> bool {
    let which = if ctx.live_prompt.showing() {
        Dialog::Live
    } else if ctx.pending_delete.showing() {
        Dialog::Delete
    } else if ctx.pending_consent.is_some() {
        Dialog::Consent
    } else if ctx.pending_pkg_install.is_some() {
        Dialog::Package
    } else {
        return menu_key(ctx, code);
    };
    match ctx.dialog_focus.key(code) {
        Some(choice) => answer(ctx, which, choice),
        None => repaint(ctx),
    }
    true
}

fn answer(ctx: &mut Context, which: Dialog, choice: Choice) {
    match which {
        Dialog::Live => super::live_prompt::answer(ctx, choice),
        Dialog::Delete => super::delete_prompt::answer(ctx, choice),
        Dialog::Consent => super::consent::answer(ctx, choice),
        Dialog::Package => super::pkg_consent::answer(ctx, choice),
    }
}

/// Esc closes the menu bar's drop-down or the desktop's right-click menu.
/// Any other key is swallowed while one is open: the menu holds the keys,
/// and a letter typed into it must not reach the window under it.
fn menu_key(ctx: &mut Context, code: u32) -> bool {
    let open = ctx.menubar.open.is_some() || ctx.desktop_menu.is_some();
    if !open {
        return false;
    }
    if code == KEY_ESC {
        ctx.menubar.open = None;
        ctx.menubar.hover = None;
        ctx.desktop_menu = None;
        ctx.menu_hover = None;
        ctx.menu_target = None;
        repaint(ctx);
    }
    true
}
