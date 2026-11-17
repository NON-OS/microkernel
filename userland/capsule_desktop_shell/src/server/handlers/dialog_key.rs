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
use nonos_toolkit::widgets::Pointer;

/// The prompts, in the order a key or a press is offered to them.
#[derive(Clone, Copy)]
pub(crate) enum Prompt {
    Live,
    Delete,
    Consent,
    Package,
}

/// The prompt that is up and takes the keys and presses, which is the one
/// drawn.
pub(crate) fn up(ctx: &Context) -> Option<Prompt> {
    if ctx.live_prompt.showing() {
        Some(Prompt::Live)
    } else if ctx.pending_delete.showing() {
        Some(Prompt::Delete)
    } else if ctx.pending_consent.is_some() {
        Some(Prompt::Consent)
    } else if ctx.pending_pkg_install.is_some() {
        Some(Prompt::Package)
    } else {
        None
    }
}

/// Whether the key went to a dialog or a menu.
pub fn key(ctx: &mut Context, code: u32) -> bool {
    let Some(which) = up(ctx) else {
        return menu_key(ctx, code);
    };
    let held = crate::render::prompts::cut(ctx);
    match ctx.dialog_focus.key_held(code, held) {
        Some(choice) => answer(ctx, which, choice),
        None => repaint(ctx),
    }
    true
}

pub(crate) fn answer(ctx: &mut Context, which: Prompt, choice: Choice) {
    match which {
        Prompt::Live => super::live_prompt::answer(ctx, choice),
        Prompt::Delete => super::delete_prompt::answer(ctx, choice),
        Prompt::Consent => super::consent::answer(ctx, choice),
        Prompt::Package => super::pkg_consent::answer(ctx, choice),
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
        ctx.menubar.pointer = Pointer::new();
        ctx.desktop_menu = None;
        ctx.menu_pointer = Pointer::new();
        ctx.menu_target = None;
        repaint(ctx);
    }
    true
}
