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
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{Rect, Scrollbar};

use crate::store::state::State;

use super::metrics::metrics;
use super::{chrome, detail, geometry, rows, status};

pub fn frame(state: &mut State, fb: &mut PaintBuffer) {
    fb.clear(roles().bg.argb());
    state.fb_w = fb.width;
    state.fb_h = fb.height;
    chrome::head(fb, state);
    chrome::tabs(fb, state);
    let (top, m) = (geometry::list_top(), metrics());

    let bottom = fb.height.saturating_sub(m.status_h + m.pad_x);
    let pane_h = bottom.saturating_sub(top);
    state.rows = (pane_h / (m.card_h + m.card_gap)).max(1) as usize;
    /* Clamped here because this is where the row count is known. */
    state.clamp_scroll();

    let list_w = geometry::list_w(fb.width);
    rows::paint(state, fb, m.pad_x, top, list_w, state.rows);
    let total = state.visible().len() as u32;
    let track_h = state.rows as u32 * (m.card_h + m.card_gap) - m.card_gap;
    let track = Rect { x: m.pad_x + list_w + m.scroll_gap, y: top, w: m.scroll_w, h: track_h };
    Scrollbar::new(state.scroll as u32, total, state.rows as u32).draw(fb, track);

    let detail_x = m.pad_x + list_w + m.pad_x;
    detail::paint(state, fb, detail_x, top, m.detail_w, pane_h);
    status::paint(fb, state);
}
