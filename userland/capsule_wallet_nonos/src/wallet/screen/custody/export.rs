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
//! Showing this account's private key, asked first.

use nonos_app_skeleton::PaintBuffer;

use super::frame::frame;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::etna::parts::value::value_block;
use crate::wallet::screen::hits;
use crate::wallet::screen::page::{edges, lead};
use crate::wallet::state::State;

const EXPORT_LEAD: &str = "Anyone who sees this key can take everything this account holds, \
     on every network. Hide it as soon as it is written down.";
const EXPORT_ASK: &str = "Anyone who sees this account's private key can take everything it \
     holds, on every network, and nothing can undo that. Show it only with no one and no \
     camera watching the screen.";

pub(super) fn export(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let status = crate::wallet::screen::status::parts(state);
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
        let key = core::str::from_utf8(&state.custody.export_hex[..]).unwrap_or("");
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
