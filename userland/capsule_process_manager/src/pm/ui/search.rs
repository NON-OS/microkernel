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
use nonos_toolkit::icons::IconId;
use nonos_toolkit::widgets::{Field, Rect};

use crate::pm::state::{Screen, State};

use super::chips;
use super::hit::Target;
use super::hover_look::look;
use super::metrics::metrics;
use super::text;

const PLACEHOLDER: &str = "Filter processes";

// The one geometry the painter and the hit test both read. The field takes the
// far right of the head band, centred in it, so the meta line can be measured
// back from its left edge instead of the two fighting over the same pixels.
// The chip strip is measured text and cannot shrink, so the field is what gives
// way when the band is tight, down to SEARCH_MIN_W and no further.
pub fn rect(fb_w: u32, screen: Screen) -> (u32, u32, u32, u32) {
    let m = metrics();
    let right =
        if super::fit::inspector(screen, fb_w) { fb_w.saturating_sub(m.inspector_w) } else { fb_w };
    let limit = right.saturating_sub(m.pane_pad_x);
    let gap = m.search_meta_gap;
    let chips_end = chips::origin(fb_w, screen).map_or(0, |o| o + chips::width() + gap);
    let w = limit.saturating_sub(chips_end).clamp(m.search_min_w, m.search_w);
    let y = m.pane_pad_top + m.head_h.saturating_sub(m.search_h) / 2;
    (limit.saturating_sub(w), y, w, m.search_h)
}

/// The toolkit field over the whole box `rect` gives the hit test, drawing
/// the magnifier inside its own body. The caret is at the end, where typing
/// adds. The field draws the pointer's hover while it is not focused.
pub fn paint(fb: &mut PaintBuffer, state: &State) {
    let (x, y, w, h) = rect(fb.width, state.screen);
    let focused = state.query_focused();
    let query = text::valid(state.query());
    Field::new(query)
        .icon(IconId::SettingsSearch)
        .placeholder(PLACEHOLDER)
        .caret(focused.then_some(query.len()))
        .state(look(state, |t| t == Target::Search, focused))
        .draw(fb, Rect { x, y, w, h });
}
