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
 * 08  REVIEW: everything the confirm button commits to, in one tile, with
 * one sentence on what becomes public. Confirm hands the request to the
 * shield service and is enabled only while one is answering.
 */

use nonos_app_skeleton::PaintBuffer;

use super::review_rows::{lead, rows};
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::etna::parts::tile::{row_height, tile, tile_row};
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::{GAP, TEXT_3};
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;
use crate::wallet::screen::hits;
use crate::wallet::state::State;

pub fn review(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let status = super::status::parts(state);
    let ready = super::absent::ready(state) && state.shield_ui.waiting.is_none();
    let footer = [("Confirm", Weight::Primary, ready), ("Edit", Weight::Secondary, true)];
    let spec = FrameSpec {
        number: "08",
        title: "Review",
        back: true,
        backdrop: Some(Backdrop::Proving),
        failure: super::absent::banner(state),
        footer: &footer,
        status: &status,
        scroll: state.scroll,
    };
    let mut l = begin(fb, &spec);
    let c = l.content;
    let text = lead(&state.shield_ui);
    let mut y = c.y
        + wrapped(fb, c.x as i32, c.y as i32, c.w as i32, Role::Lead, text, TEXT_3) as u32
        + GAP;
    let all = rows(&state.shield_ui);
    let at = Rect::new(c.x, y, c.w, row_height() * all.len() as u32);
    tile(fb, at);
    for (i, (name, value)) in all.iter().enumerate() {
        tile_row(fb, at, y + row_height() * i as u32, name, value, i + 1 == all.len());
    }
    y += at.h;
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    super::edges::put(state, &l);
    super::edges::footer(&l, &[ready, true]);
}
