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
 * 05  DEPOSIT: a public payment from this account into the pool, in one of
 * the standard sizes. What it reveals is said before the size is chosen,
 * and when the note it makes can first be spent is said after.
 */

use nonos_app_skeleton::PaintBuffer;

use super::consts::POOL;
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::etna::parts::fact::fact;
use crate::wallet::etna::tokens::{GAP, TEXT_3};
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;
use crate::wallet::screen::hits;
use crate::wallet::state::shield_ui::ASSET_NOX;
use crate::wallet::state::State;

const LEAD: &str = "A deposit is public. Anyone can see that this account paid a \
     standard amount into the pool; nobody can see where it goes next.";

pub fn deposit(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let ui = &state.shield_ui;
    let status = super::status::parts(state);
    let ready = super::check::deposit_ready(ui);
    let footer = [("Review deposit", Weight::Primary, ready)];
    let spec = FrameSpec {
        number: "05",
        title: "Deposit",
        back: true,
        backdrop: Some(Backdrop::Deposit),
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
    y += fact(fb, c.x, y, c.w, "pool", POOL);
    y += fact(fb, c.x, y, c.w, "spendable after", "20 later notes and 1,800 blocks");
    if ui.asset == ASSET_NOX {
        y += fact(fb, c.x, y, c.w, "first", "one approval for the pool");
    }
    y += super::held::held(fb, c, y, ui, crate::wallet::state::shield_ui::SHIELD_DEPOSIT);
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    super::edges::put(state, &l);
    super::edges::footer(&l, &[ready]);
}
