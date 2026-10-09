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
 * 04  SHIELD on a network without the pool. Nothing from the shield is
 * shown here, since none of it is this network's: only that the pool is
 * on Sepolia, and the one press that goes there.
 */

use nonos_app_skeleton::PaintBuffer;

use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::etna::tokens::{GAP, TEXT_3};
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;
use crate::wallet::screen::hits;
use crate::wallet::state::State;

const MORE: &str = "Your public balances on this network are on the wallet's home screen. \
     Switching to Sepolia closes nothing there: the same address holds both.";

pub fn elsewhere(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let status = super::status::parts(state);
    let footer = [("Switch to Sepolia", Weight::Primary, true)];
    let spec = FrameSpec {
        number: "04",
        title: "Shield",
        back: true,
        backdrop: Some(Backdrop::Deposit),
        failure: state.failure,
        footer: &footer,
        status: &status,
        scroll: state.scroll,
    };
    let mut l = begin(fb, &spec);
    let c = l.content;
    let lead = crate::wallet::shield::open::ELSEWHERE;
    let mut y = c.y
        + wrapped(fb, c.x as i32, c.y as i32, c.w as i32, Role::Lead, lead, TEXT_3) as u32
        + GAP;
    y += wrapped(fb, c.x as i32, y as i32, c.w as i32, Role::Lead, MORE, TEXT_3) as u32 + GAP;
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    super::edges::put(state, &l);
    super::edges::footer(&l, &[true]);
}
