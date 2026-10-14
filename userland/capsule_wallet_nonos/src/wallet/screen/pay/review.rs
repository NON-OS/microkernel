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
//! The review: every field the confirm signs, and the one sentence on what
//! becomes public. Confirm is the only thing that signs.

use nonos_app_skeleton::PaintBuffer;

use super::super::hits;
use super::super::page::{edges, lead};
use super::review_rows::draft;
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::state::State;

const LEAD: &str = "Check it before it goes. A payment cannot be taken back once the \
     network has it.";

pub fn review(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let status = super::super::status::parts(state);
    /* While the payment goes out the buttons say so and take no press. */
    let working = crate::wallet::act::working(state);
    let ready = state.send.draft.is_some() && working.is_none();
    let footer = [
        (working.unwrap_or("Confirm and send"), Weight::Primary, ready),
        ("Edit", Weight::Secondary, working.is_none()),
    ];
    let spec = FrameSpec {
        number: "05",
        title: "Review",
        back: true,
        backdrop: Some(Backdrop::Proving),
        failure: state.failure,
        footer: &footer,
        status: &status,
        scroll: state.scroll,
    };
    let mut l = begin(fb, &spec);
    let c = l.content;
    let mut y = lead(fb, c, LEAD);
    if let Some(d) = &state.send.draft {
        y = draft(fb, c, y, d);
    }
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    edges(&l, &[ready, working.is_none()]);
}
