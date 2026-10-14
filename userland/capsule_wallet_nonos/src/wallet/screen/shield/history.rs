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
 * 10  HISTORY: every deposit, payment, receipt and withdrawal the shield
 * service has recorded, each with the state it has reached. Nothing is
 * listed that the service did not report.
 */

use alloc::format;
use nonos_app_skeleton::PaintBuffer;

use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::tokens::{GAP, TEXT_3};
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;
use crate::wallet::screen::hits;
use crate::wallet::state::State;

pub fn history(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let status = super::status::parts(state);
    let spec = FrameSpec {
        number: "10",
        title: "History",
        back: true,
        backdrop: Some(Backdrop::History),
        failure: super::absent::banner(state),
        footer: &[],
        status: &status,
        scroll: state.scroll,
    };
    let mut l = begin(fb, &spec);
    let c = l.content;
    let log = &state.shield_ui.history;
    let mut y = c.y;
    y = super::history_rows::rows(fb, c, y, log);
    let cut = state.shield_ui.history_cut;
    if cut > 0 {
        let note = format!(
            "{cut} older entries are kept in the shield store. Only the newest that fit \
             in one reply are shown here."
        );
        y += GAP / 2;
        y += wrapped(fb, c.x as i32, y as i32, c.w as i32, Role::Lead, &note, TEXT_3) as u32;
    }
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    super::edges::put(state, &l);
}
