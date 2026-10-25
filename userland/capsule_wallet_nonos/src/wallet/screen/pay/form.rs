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
//! The payment form: what to send, to whom, and what this account holds.

use nonos_app_skeleton::PaintBuffer;

use super::super::hits;
use super::super::page::{edges, lead};
use super::form_fields::fields;
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::state::State;

const LEAD: &str = "Public: the amount, both addresses and the time are visible to anyone. \
     For a private payment, use Shield.";

pub fn ready(state: &State) -> bool {
    state.send.to_len == 40 && !state.send.amount.is_zero()
}

pub fn form(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let status = super::super::status::parts(state);
    /* While the network is read for the review the button says so. */
    let working = crate::wallet::act::working(state);
    let ok = ready(state) && working.is_none();
    let all = working.is_none() && crate::wallet::send::held(state, state.send.token).is_some();
    let footer = [
        (working.unwrap_or("Review payment"), Weight::Primary, ok),
        ("Use all", Weight::Secondary, all),
    ];
    let spec = FrameSpec {
        number: "05",
        title: "Send",
        back: true,
        backdrop: Some(Backdrop::Send),
        failure: state.failure,
        footer: &footer,
        status: &status,
        scroll: state.scroll,
    };
    let mut l = begin(fb, &spec);
    let c = l.content;
    let mut y = lead(fb, c, LEAD);
    y = fields(state, fb, c, y);
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    edges(&l, &[ok, all]);
}
