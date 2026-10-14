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

//! The settings screen, drawn: its lead, its facts and the key list.

use nonos_app_skeleton::PaintBuffer;

use super::facts::facts;
use super::words::LEAD;
use super::ROWS;
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::quiet::quiet_group;
use crate::wallet::etna::rect::Rect;
use crate::wallet::screen::hits::{self, Press};
use crate::wallet::screen::page::{edges, lead};
use crate::wallet::state::State;

pub fn show(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let status = crate::wallet::screen::status::parts(state);
    let spec = FrameSpec {
        number: "09",
        title: "Settings",
        back: true,
        backdrop: Some(Backdrop::Settings),
        failure: state.failure,
        footer: &[],
        status: &status,
        scroll: state.scroll,
    };
    let mut l = begin(fb, &spec);
    let c = l.content;
    let mut y = lead(fb, c, LEAD);
    y = facts(state, fb, c, y);
    let rows: [(&str, bool); 6] = [
        (ROWS[0], state.acct.address_ready),
        (ROWS[1], true),
        (ROWS[2], true),
        (ROWS[3], state.acct.address_ready),
        (ROWS[4], state.acct.address_ready),
        (ROWS[5], state.acct.address_ready),
    ];
    let mut at = [Rect::new(0, 0, 0, 0); 6];
    y += quiet_group(fb, c.x, y, c.w, &rows, &mut at);
    for (i, r) in at.iter().enumerate() {
        if rows[i].1 {
            hits::put(Press::Row(i as u8), *r);
        }
    }
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    edges(&l, &[]);
}
