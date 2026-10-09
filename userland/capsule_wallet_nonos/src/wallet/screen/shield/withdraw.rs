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
 * 07  WITHDRAW: a standard amount leaves the pool for a fresh public
 * address of this wallet. The proof is built here and a relayer submits
 * it, so the fee is named, and the spend wait is shown before anything is.
 */

use nonos_app_skeleton::PaintBuffer;

use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::etna::parts::fact::fact;
use crate::wallet::etna::tokens::{GAP, TEXT_3};
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;
use crate::wallet::screen::hits;
use crate::wallet::state::State;

const LEAD: &str = "A withdrawal is public: a standard amount leaves the pool for a \
     fresh address of this wallet, never one it has used before.";

pub fn withdraw(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let ui = &state.shield_ui;
    let status = super::status::parts(state);
    let ready = super::check::withdraw_ready(ui);
    let footer = [("Review withdrawal", Weight::Primary, ready)];
    let spec = FrameSpec {
        number: "07",
        title: "Withdraw",
        back: true,
        backdrop: Some(Backdrop::Withdraw),
        failure: super::absent::banner(state),
        footer: &footer,
        status: &status,
        scroll: state.scroll,
    };
    let mut l = begin(fb, &spec);
    let c = l.content;
    let mut y = c.y
        + wrapped(fb, c.x as i32, c.y as i32, c.w as i32, Role::Lead, LEAD, TEXT_3) as u32
        + GAP;
    y += super::pick::asset(fb, c, y, ui);
    y += super::pick::size(fb, c, y, ui);
    y += fact(fb, c.x, y, c.w, "fee", "read from the pool at review");
    y += fact(fb, c.x, y, c.w, "to", "a fresh address of this wallet");
    y += GAP / 2 + super::meter::meter(fb, c, y + GAP / 2, ui);
    y += super::send_fields::early(fb, c, y, ui);
    y += super::held::held(fb, c, y, ui, crate::wallet::state::shield_ui::SHIELD_WITHDRAW);
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    super::edges::put(state, &l);
    super::edges::footer(&l, &[ready]);
}
