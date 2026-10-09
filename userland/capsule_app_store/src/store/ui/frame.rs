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

//! One frame: ground, head, tabs, list, detail, status.

use nonos_app_skeleton::PaintBuffer;

use crate::store::state::State;
use crate::store::theme::BACKGROUND;

use super::metrics::{CARD_GAP, CARD_H, DETAIL_W, PAD_X, STATUS_H};
use super::{chrome, detail, geometry, rows, scrollbar, status};

pub fn frame(state: &mut State, fb: &mut PaintBuffer) {
    fb.clear(BACKGROUND);
    state.fb_w = fb.width;
    state.fb_h = fb.height;
    chrome::head(fb, state);
    chrome::tabs(fb, state);
    let top = geometry::list_top();

    let bottom = fb.height.saturating_sub(STATUS_H + PAD_X);
    let pane_h = bottom.saturating_sub(top);
    state.rows = (pane_h / (CARD_H + CARD_GAP)).max(1) as usize;
    // Clamped here because this is where the row count is known.
    state.clamp_scroll();

    let list_w = geometry::list_w(fb.width);
    rows::paint(state, fb, PAD_X, top, list_w, state.rows);
    let total = state.visible().len();
    scrollbar::paint(fb, PAD_X, top, list_w, state.rows, total, state.scroll);

    let detail_x = PAD_X + list_w + PAD_X;
    detail::paint(state, fb, detail_x, top, DETAIL_W, pane_h);
    status::paint(fb, state);
}
