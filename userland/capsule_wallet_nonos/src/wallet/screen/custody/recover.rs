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
//! Restoring from recovery words, shown as dots unless asked.

use alloc::format;
use alloc::string::String;

use nonos_app_skeleton::PaintBuffer;

use super::frame::frame;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::etna::parts::fact::fact;
use crate::wallet::etna::tokens::GAP;
use crate::wallet::screen::hits::{self, Press};
use crate::wallet::screen::page::{edges, lead};
use crate::wallet::screen::shield::parts::field;
use crate::wallet::state::State;

const RECOVER_LEAD: &str = "Type the 12 to 24 words in order, separated by spaces. The same \
     words restore this account in any Ethereum wallet. They show as dots unless you ask \
     to see them.";

pub(super) fn recover(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let status = crate::wallet::screen::status::parts(state);
    let typed =
        core::str::from_utf8(&state.custody.recover_buf[..state.custody.recover_len]).unwrap_or("");
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
