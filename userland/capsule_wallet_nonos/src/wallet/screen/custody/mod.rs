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
//! 02  The key itself: importing a private key, restoring from recovery
//! words, and showing this account's private key. Each owns the keyboard
//! while it is up, and what is typed or shown is wiped when it closes.

use alloc::format;
use alloc::string::String;

use nonos_app_skeleton::PaintBuffer;

use super::hits::{self, Press};
use super::page::{edges, lead};
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::etna::parts::fact::fact;
use crate::wallet::etna::parts::value::value_block;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::GAP;
use crate::wallet::screen::shield::parts::field;
use crate::wallet::state::{State, VIEW_EXPORT, VIEW_IMPORT};

pub mod click;

const IMPORT_LEAD: &str = "The key goes straight to the keyring and is wiped from this screen. \
     It is never drawn: each digit shows as a dot.";
/* C2: the sealed copy opens only under this machine's kernel and firmware. */
const IMPORT_KEEP: &str = "A key has no recovery words. This machine keeps it sealed to its \
     kernel and firmware, so after an update, or on another machine, the sealed copy cannot \
     be opened, and only the key itself brings the account back. Keep it written down, \
     offline, before you import it.";
const RECOVER_LEAD: &str = "Type the 12 to 24 words in order, separated by spaces. The same \
     words restore this account in any Ethereum wallet. They show as dots unless you ask \
     to see them.";
const EXPORT_LEAD: &str = "Anyone who sees this key can take everything this account holds, \
     on every network. Hide it as soon as it is written down.";
const EXPORT_ASK: &str = "Anyone who sees this account's private key can take everything it \
     holds, on every network, and nothing can undo that. Show it only with no one and no \
     camera watching the screen.";

pub fn show(state: &State, fb: &mut PaintBuffer) {
    match state.view {
        VIEW_IMPORT => import(state, fb),
        VIEW_EXPORT => export(state, fb),
        _ => recover(state, fb),
    }
}

fn frame<'a>(
    state: &'a State,
    title: &'a str,
    footer: &'a [(&'a str, Weight, bool)],
    status: &'a [&'a str],
) -> FrameSpec<'a> {
    FrameSpec {
        number: "02",
        title,
        back: true,
        backdrop: Some(Backdrop::Backup),
        failure: state.failure,
        footer,
        status,
        scroll: state.scroll,
    }
}

fn import(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let status = super::status::parts(state);
    let ok = state.import_len == 64;
    let footer = [("Import", Weight::Primary, ok), ("Cancel", Weight::Secondary, true)];
    let spec = frame(state, "Import a key", &footer, &status);
    let mut l = begin(fb, &spec);
    let c = l.content;
    let mut y = lead(fb, c, IMPORT_LEAD);
    let dots: String = core::iter::repeat('\u{2022}').take(state.import_len).collect();
    let (at, h) = field(fb, c, y, "PRIVATE KEY", &dots, "64 hex digits, 0x optional", true);
    hits::put(Press::Field(0), at);
    y += h + GAP;
    y += fact(fb, c.x, y, c.w, "typed", &format!("{} of 64", state.import_len)) + GAP;
    y = lead(fb, Rect::new(c.x, y, c.w, 0), IMPORT_KEEP);
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    edges(&l, &[ok, true]);
}

fn recover(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let status = super::status::parts(state);
    let typed = core::str::from_utf8(&state.recover_buf[..state.recover_len]).unwrap_or("");
    let words = typed.split(' ').filter(|w| !w.is_empty()).count();
    let ok = matches!(words, 12 | 15 | 18 | 21 | 24);
    /* Each letter a dot, the spaces kept, so the words can be counted. */
    let masked: String = typed.chars().map(|c| if c == ' ' { ' ' } else { '\u{2022}' }).collect();
    let drawn = if state.recover_shown { typed } else { masked.as_str() };
    let toggle = if state.recover_shown { "Hide the words" } else { "Show the words" };
    let footer = [
        ("Restore", Weight::Primary, ok),
        ("Cancel", Weight::Secondary, true),
        (toggle, Weight::Secondary, true),
    ];
    let spec = frame(state, "Restore", &footer, &status);
    let mut l = begin(fb, &spec);
    let c = l.content;
    let mut y = lead(fb, c, RECOVER_LEAD);
    let (at, h) = field(fb, c, y, "RECOVERY WORDS", drawn, "word word word ...", true);
    hits::put(Press::Field(0), at);
    y += h + GAP;
    y += fact(fb, c.x, y, c.w, "words", &format!("{words}"));
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    edges(&l, &[ok, true, true]);
}

fn export(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let status = super::status::parts(state);
    /* Asked first: the key is fetched only by the second screen's button. */
    let shown = state.export_active;
    let footer = if shown {
        [("Hide the key", Weight::Primary, true), ("Cancel", Weight::Secondary, false)]
    } else {
        [("Show the key", Weight::Primary, true), ("Cancel", Weight::Secondary, true)]
    };
    let footer = if shown { &footer[..1] } else { &footer[..] };
    let spec = frame(state, "Private key", footer, &status);
    let mut l = begin(fb, &spec);
    let c = l.content;
    let mut y = lead(fb, c, if shown { EXPORT_LEAD } else { EXPORT_ASK });
    if shown {
        let key = core::str::from_utf8(&state.export_hex).unwrap_or("");
        y += value_block(fb, c.x, y, c.w, key);
    }
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    if shown {
        edges(&l, &[true]);
    } else {
        edges(&l, &[true, true]);
    }
}
