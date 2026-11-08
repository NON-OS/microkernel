// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The prompt that is up, drawn as the shared `Dialog` over the whole chrome
//! layer. The paint pass, the pointer's light and the click all build it
//! here from the same words, so a button lights and draws where a press on
//! it lands.

use alloc::string::String;

use nonos_toolkit::widgets::{Dialog, Rect};

use super::menu_view::bounds;
use super::prompt_buttons::{armed, Buttons};
use super::prompt_view::{draw, lit as view_lit};
use super::surface::surface;
use super::whole_prompt::{held, whole};
use crate::server::handlers::dialog_key::{up, Prompt};
use crate::state::dialog_keys::Choice;
use crate::state::Context;

/// A prompt's title, its body, and its two buttons, the one that acts first.
/// With `warn_cut`, the body must be read whole: it takes every line the
/// screen holds, and a body the panel still cuts says so in a warning note.
pub(super) struct Words {
    pub(super) title: String,
    pub(super) body: String,
    pub(super) buttons: Buttons,
    pub(super) warn_cut: Option<&'static str>,
}

impl Words {
    fn dialog(&self, at: Rect) -> Dialog<'_> {
        let d = Dialog::new(&self.title).body(&self.body).buttons(&self.buttons);
        let d = d.armed(armed(&self.buttons));
        match self.warn_cut {
            Some(note) => whole(d, note, at),
            None => d,
        }
    }

    /// The dialog as drawn and lit: one that must be read whole holds its
    /// acting button, disabled, while it is cut.
    fn seen(&self, at: Rect) -> Dialog<'_> {
        let d = self.dialog(at);
        if self.warn_cut.is_some() {
            held(d, at)
        } else {
            d
        }
    }
}

fn shown(ctx: &Context) -> Option<(Prompt, Words, Rect)> {
    let which = up(ctx)?;
    let words = match which {
        Prompt::Live => Some(super::live_prompt::words()),
        Prompt::Delete => super::delete_prompt::words(ctx),
        Prompt::Consent => super::consent::words(ctx),
        Prompt::Package => super::pkg_consent::words(ctx),
    }?;
    Some((which, words, bounds(ctx.width, ctx.height)))
}

/// Paints the prompt that is up, ringing the button the keyboard is on
/// unless it is the acting one, drawn disabled while the prompt holds it.
pub fn paint(ctx: &Context) {
    let Some((_, words, at)) = shown(ctx) else {
        return;
    };
    let focus = if ctx.dialog_focus.focused() == Choice::Act { 0 } else { 1 };
    draw(&mut surface(ctx), words.seen(at).focus(focus), at, &ctx.prompt_pointer);
}

/// The prompt that is up, and the index of its button under the point.
pub fn hit(ctx: &Context, x: i32, y: i32) -> Option<(Prompt, Option<usize>)> {
    let (which, words, at) = shown(ctx)?;
    Some((which, words.dialog(at).hit(at, x, y)))
}

/// Whether the prompt that is up holds its acting button: its body must be
/// read whole, as the package consent's capability list must, and the
/// `Dialog` drawn here still cuts it. Approve then answers nothing.
pub fn cut(ctx: &Context) -> bool {
    shown(ctx).is_some_and(|(_, words, at)| words.warn_cut.is_some() && words.dialog(at).cut(at))
}

/// The buttons the shell's pointer hovers and holds on the prompt that is up.
pub fn lit(ctx: &Context) -> (Option<usize>, Option<usize>) {
    shown(ctx)
        .map_or((None, None), |(_, words, at)| view_lit(&words.seen(at), at, &ctx.prompt_pointer))
}
