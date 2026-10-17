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

//! A publisher transaction's review (B8): the decoded call in full, then
//! Sign and send, lit only at the review, once its rows read back and the
//! guard is armed (arm::gate::greyed, as the press asks it).

use nonos_app_skeleton::PaintBuffer;

use super::review_say::{lead as said, page};
use crate::wallet::arm::gate::greyed;
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::etna::parts::value::value_block;
use crate::wallet::etna::tokens::GAP;
use crate::wallet::hex_id::hex;
use crate::wallet::intent::IntentStep;
use crate::wallet::publish::sending::Sending;
use crate::wallet::screen::hits::{self, Press};
use crate::wallet::screen::page::{edges, lead};
use crate::wallet::state::State;

pub fn review(state: &State, s: &Sending, fb: &mut PaintBuffer) {
    hits::clear();
    let status = crate::wallet::screen::status::parts(state);
    let rows = page(s);
    let review = matches!(s.step, IntentStep::Review { .. });
    let sign = review && !s.pressed && rows.is_some() && !greyed(state, Press::Footer(0));
    let ended = matches!(s.step, IntentStep::Landed | IntentStep::Failed(_));
    let footer = [
        ("Sign and send", Weight::Primary, sign),
        (if ended { "Close" } else { "Cancel" }, Weight::Secondary, true),
    ];
    let spec = FrameSpec {
        number: "11",
        title: "Review",
        back: true,
        backdrop: Some(Backdrop::Settings),
        failure: state.failure,
        footer: &footer,
        status: &status,
        scroll: state.scroll,
    };
    let mut l = begin(fb, &spec);
    let c = l.content;
    let mut y = lead(fb, c, &said(s, !review || rows.is_some(), &state.broadcast.sent));
    if let IntentStep::Sent(h) = s.step {
        y += value_block(fb, c.x, y, c.w, &hex(&h)) + GAP;
        y += super::review_echo::echo(fb, c, y, s.echo);
    }
    if let Some(rows) = &rows {
        y += super::review_rows::draw(fb, c, y, rows);
    }
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    edges(&l, &[sign, true]);
}
