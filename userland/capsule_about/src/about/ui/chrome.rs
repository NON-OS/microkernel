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

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::text::{centre_top, left, right, width};
use nonos_toolkit::widgets::Rect;

use crate::about::state::State;

use super::metrics::metrics;
use super::mono::valid;

pub use super::status_bar::paint as status_bar;

// The content box for the active screen: everything right of the sidebar, below
// the page head, above the status bar. Every screen painter takes this rect and
// never recomputes it.
pub fn pane_rect(fb_w: u32, fb_h: u32) -> Rect {
    let m = metrics();
    Rect {
        x: m.sidebar_w + m.pane_pad_x,
        y: m.pane_pad_top + m.head_h,
        w: fb_w.saturating_sub(m.sidebar_w + m.pane_pad_x * 2),
        h: fb_h.saturating_sub(m.pane_pad_top + m.head_h + m.status_h + m.pane_pad_x),
    }
}

// The section name holds the left of the head band and its meta line the right,
// measured back from the pane's right edge so a longer name never pushes it off
// the window. If the two would collide the meta yields: the name identifies the
// screen, the meta only describes it.
pub fn page_head(fb: &mut PaintBuffer, state: &State) {
    let (r, m) = (roles(), metrics());
    let (title_px, body_px, x) = (m.title_px, m.body_px, m.sidebar_w + m.pane_pad_x);
    let title_top = centre_top(m.pane_pad_top, m.head_h, title_px);
    let label = valid(state.section.nav_label());
    left(fb, x, title_top, label, r.text, title_px);
    let meta = valid(state.section.head_meta());
    let meta_top = centre_top(m.pane_pad_top, m.head_h, body_px);
    let right_x = state.fb_w.saturating_sub(m.pane_pad_x);
    let title_end = x + width(label, title_px) + m.head_meta_gap;
    if title_end + width(meta, body_px) <= right_x {
        right(fb, right_x, meta_top, meta, r.muted, body_px);
    }
}
