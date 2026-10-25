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

/*
 * 03  RECEIVE, as the phone app has it: the private nox1 address first,
 * whose payments nobody can link to this account's balance or notes, and
 * the public 0x address one press away. The QR is drawn large enough for a
 * phone to read the 2,009 letters.
 */

use nonos_app_skeleton::PaintBuffer;

use super::hits::{self, Press};
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::state::State;

pub fn receive(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let status = super::status::parts(state);
    /* Private only where the shield pool runs, whichever way this screen
     * was reached: off Sepolia nothing of the shield is shown here. */
    let here = crate::wallet::shield::open::here();
    let private = state.acct.receive_private && here;
    let nox1 = state.shield_ui.nox1.as_deref().filter(|_| private);
    let ready = state.acct.address_ready && (!private || nox1.is_some());
    let other = if private { "Receive at the 0x address" } else { "Receive privately" };
    let can_switch = state.acct.address_ready && here;
    let footer = [("Copy address", Weight::Primary, ready), (other, Weight::Secondary, can_switch)];
    let spec = FrameSpec {
        number: "03",
        title: "Receive",
        back: true,
        backdrop: Some(Backdrop::Receive),
        failure: None,
        footer: &footer,
        status: &status,
        scroll: state.scroll,
    };
    let mut l = begin(fb, &spec);
    let c = l.content;
    let mut y = super::receive_body::body(state, fb, c, private, here, nox1);
    y += super::receive_address::facts(state, fb, c, y);
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    if let Some(back) = l.back {
        hits::put(Press::Back, back);
    }
    if ready {
        hits::put(Press::Footer(0), l.footer[0]);
    }
    if can_switch {
        hits::put(Press::Footer(1), l.footer[1]);
    }
}
