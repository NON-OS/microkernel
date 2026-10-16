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
 * 09  PROVING: the proof being built on this machine and how long it has
 * taken, then the spend's way to the chain: handed to a lander, waiting,
 * landed. When no lander takes it, the holder may settle it from their own
 * account, told first what that reveals, or take the notes back.
 */

use alloc::vec::Vec;
use nonos_app_skeleton::PaintBuffer;

pub use super::proving_buttons::{
    buttons, earlier_to_settle, SETTLE, SETTLE_EARLIER, STOP, TAKE_BACK,
};
pub use super::proving_job::doing;
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::screen::hits;
use crate::wallet::state::State;

pub fn proving(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let ui = &state.shield_ui;
    let status = super::status::parts(state);
    let shown = buttons(ui);
    let footer: Vec<(&str, Weight, bool)> = shown
        .iter()
        .enumerate()
        .map(|(i, (t, _))| (*t, if i == 0 { Weight::Primary } else { Weight::Secondary }, true))
        .collect();
    let spec = FrameSpec {
        number: "09",
        title: "Proving",
        back: true,
        backdrop: Some(Backdrop::Proving),
        failure: super::absent::banner(state),
        footer: &footer,
        status: &status,
        scroll: state.scroll,
    };
    let mut l = begin(fb, &spec);
    let c = l.content;
    let mut y = c.y;
    y = super::proving_job::job(fb, ui, c, y);
    y = super::proving_spend::spend(fb, ui, c, y);
    y = super::proving_earlier::earlier(fb, ui, c, y);
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    super::edges::put(&l);
    let on: Vec<bool> = shown.iter().map(|_| true).collect();
    super::edges::footer(&l, &on);
}
