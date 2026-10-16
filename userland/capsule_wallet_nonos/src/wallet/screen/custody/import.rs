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
//! Importing a private key: never drawn, each digit a dot.

use alloc::format;

use nonos_app_skeleton::PaintBuffer;

use super::frame::frame;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::etna::parts::fact::fact;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::GAP;
use crate::wallet::screen::hits::{self, Press};
use crate::wallet::screen::page::{edges, lead};
use crate::wallet::screen::shield::parts::field;
use crate::wallet::state::State;

const IMPORT_LEAD: &str = "The key goes straight to the keyring and is wiped from this screen. \
     It is never drawn: each digit shows as a dot.";
/* C2: the sealed copy opens only under this machine's kernel and firmware. */
const IMPORT_KEEP: &str = "A key has no recovery words. This machine keeps it sealed to its \
     kernel and firmware, so after an update, or on another machine, the sealed copy cannot \
     be opened, and only the key itself brings the account back. Keep it written down, \
     offline, before you import it.";

pub(super) fn import(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let status = crate::wallet::screen::status::parts(state);
    let ok = state.custody.import_len == 64;
    let footer = [("Import", Weight::Primary, ok), ("Cancel", Weight::Secondary, true)];
    let spec = frame(state, "Import a key", &footer, &status);
    let mut l = begin(fb, &spec);
    let c = l.content;
    let mut y = lead(fb, c, IMPORT_LEAD);
    let dots = "\u{2022}".repeat(state.custody.import_len);
    let (at, h) = field(fb, c, y, "PRIVATE KEY", &dots, "64 hex digits, 0x optional", true);
    hits::put(Press::Field(0), at);
    y += h + GAP;
    y += fact(fb, c.x, y, c.w, "typed", &format!("{} of 64", state.custody.import_len)) + GAP;
    y = lead(fb, Rect::new(c.x, y, c.w, 0), IMPORT_KEEP);
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    edges(&l, &[ok, true]);
}
