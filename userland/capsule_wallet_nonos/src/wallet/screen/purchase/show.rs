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

//! A purchase the store named (B8, the plan): the market's terms, Buy or
//! Close, then each step as the meter takes it.

use nonos_app_skeleton::PaintBuffer;

use super::rows::rows;
use super::words::{closable, lead as first};
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::etna::parts::fact::fact;
use crate::wallet::screen::hits;
use crate::wallet::screen::page::{edges, lead};
use crate::wallet::state::State;

pub fn show(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let Some(p) = state.meter.purchase.as_deref() else { return };
    let (sentence, action) = first(p);
    let (close, act) = (closable(p), action.is_some());
    let status = crate::wallet::screen::status::parts(state);
    let footer =
        [("Close", Weight::Secondary, close), (action.unwrap_or(""), Weight::Primary, act)];
    let spec = FrameSpec {
        number: "13",
        title: "Purchase",
        back: false,
        backdrop: Some(Backdrop::Send),
        failure: state.failure,
        footer: &footer,
        status: &status,
        scroll: state.scroll,
    };
    let mut l = begin(fb, &spec);
    let c = l.content;
    let mut y = lead(fb, c, sentence);
    for (label, value) in rows(p) {
        y += fact(fb, c.x, y, c.w, label, &value);
    }
    if super::form::open_form(p) {
        y += super::form::paint(p, fb, c, y);
    }
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    edges(&l, &[close, act]);
}
