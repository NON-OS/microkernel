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
 * 04  SHIELD, the private side of the wallet: what the pool holds for this
 * account, the three things a holder does with it, the address others pay
 * privately, and the way to the history and the network the pool is on.
 */

use nonos_app_skeleton::PaintBuffer;

use super::home_more::more;
use super::home_parts::{actions, balances, receive};
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::tokens::{GAP, TEXT_3};
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;
use crate::wallet::screen::hits;
use crate::wallet::state::State;

const LEAD: &str = "Shielded value lives in the pool as notes only this wallet can open.";

pub fn home(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let status = super::status::parts(state);
    let spec = FrameSpec {
        number: "04",
        title: "Shield",
        back: true,
        backdrop: Some(Backdrop::Deposit),
        failure: super::absent::banner(state),
        footer: &[],
        status: &status,
        scroll: state.scroll,
    };
    let mut l = begin(fb, &spec);
    let c = l.content;
    let mut y = c.y
        + wrapped(fb, c.x as i32, c.y as i32, c.w as i32, Role::Lead, LEAD, TEXT_3) as u32
        + GAP;
    y += balances(state, fb, c, y) + GAP;
    y += actions(fb, c, y) + GAP;
    y += receive(state, fb, c, y) + GAP;
    y += more(state, fb, c, y);
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    super::edges::put(state, &l);
}
