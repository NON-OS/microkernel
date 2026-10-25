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

//! A release's attest under review: every row it signs, the digest the
//! keyring will sign, the account that signs and the Bond that relies on
//! it, then Sign the attest, lit only once the rows are there and the
//! guard is armed (arm::gate::greyed).

use nonos_app_skeleton::PaintBuffer;

use super::click_attest::review_of;
use crate::wallet::arm::gate::greyed;
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::screen::hits::{self, Press};
use crate::wallet::screen::page::{edges, lead};
use crate::wallet::state::State;

const LEAD: &str = "Anyone who installs this release relies on this attest, and the Bond \
     slashes a second, different one. Sign it only if every row is yours.";

pub fn show(state: &State, i: usize, fb: &mut PaintBuffer) {
    hits::clear();
    let status = crate::wallet::screen::status::parts(state);
    let review = review_of(state, i);
    let same = review.as_ref().ok().map(|r| r.digest) == state.publisher.attest_digest;
    let sign = review.is_ok() && same && !greyed(state, Press::Footer(0));
    let footer = [("Sign the attest", Weight::Primary, sign), ("Cancel", Weight::Secondary, true)];
    let spec = FrameSpec {
        number: "11",
        title: "Review the attest",
        back: true,
        backdrop: Some(Backdrop::Settings),
        failure: state.failure,
        footer: &footer,
        status: &status,
        scroll: state.scroll,
    };
    let mut l = begin(fb, &spec);
    let c = l.content;
    let mut y = match &review {
        Ok(_) => lead(fb, c, LEAD),
        Err(why) => lead(fb, c, why),
    };
    if let Ok(r) = &review {
        y += super::review_rows::draw(fb, c, y, &r.rows);
    }
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    edges(&l, &[sign, true]);
}
