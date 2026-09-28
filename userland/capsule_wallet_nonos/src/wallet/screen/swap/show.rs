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
 * 12  SWAP: what goes in, what comes back, and every term of the trade in
 * full before anything is signed. The quote is the pool's own reading or
 * nothing at all; a trade with no price says so rather than showing zero.
 */

use nonos_app_skeleton::PaintBuffer;

use super::pair::{pay, receive};
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::etna::tokens::{GAP, TEXT_3};
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;
use crate::wallet::screen::hits::{self, Press};
use crate::wallet::state::State;

const LEAD: &str = "A swap is a public trade on Ethereum mainnet. Type an amount; the \
     rate, price impact and minimum received are shown before you sign.";

pub fn show(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let status = super::super::status::parts(state);
    let ready = state.swap_quote.ready;
    let footer = [(super::click::action(state), Weight::Primary, ready)];
    let spec = FrameSpec {
        number: "12",
        title: "Swap",
        back: true,
        backdrop: Some(Backdrop::Send),
        failure: super::click::failure(state),
        footer: &footer,
        status: &status,
        scroll: state.scroll,
    };
    let mut l = begin(fb, &spec);
    let c = l.content;
    let mut y = c.y
        + wrapped(fb, c.x as i32, c.y as i32, c.w as i32, Role::Lead, LEAD, TEXT_3) as u32
        + GAP;
    y += pay(state, fb, c, y);
    y += receive(state, fb, c, y);
    y += super::terms::terms(state, fb, c, y);
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    if let Some(back) = l.back {
        hits::put(Press::Back, back);
    }
    if ready {
        hits::put(Press::Footer(0), l.footer[0]);
    }
    if let Some(d) = l.dismiss {
        hits::put(Press::Dismiss, d);
    }
}
