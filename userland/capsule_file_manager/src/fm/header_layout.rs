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

extern crate alloc;

use alloc::{string::String, vec::Vec};

use nonos_app_skeleton::measure_ttf;

use super::header_slots::{
    HeadHit, Slot, GRID_LABEL, LIST_LABEL, SEARCH_W, TOOL_GAP, TOOL_H, TOOL_PAD, TOOL_PX,
    UNDO_LABEL,
};
use super::layout::{HEADER_H, PAD_X};
use super::state::State;

/// Top of the control band, centred in the header below the window titlebar.
pub fn tool_y() -> u32 {
    (HEADER_H - TOOL_H) / 2 + 6
}

/// The sort control's label. Measured and drawn from this one string.
pub fn sort_text(state: &State) -> String {
    let mut text = String::from("Sort: ");
    text.push_str(core::str::from_utf8(state.sort_mode.label()).unwrap_or("?"));
    text
}

/// Width of a labelled pill at the strip's em size.
pub fn pill_w(label: &str) -> u32 {
    (measure_ttf(label, TOOL_PX).max(0) as u32) + TOOL_PAD * 2
}

/// The control strip, laid out right-to-left from the window edge and returned
/// left-to-right. `paint_toolbar` draws these and `head_hit` tests them, so a
/// control is only ever clickable where it was actually drawn. The two view
/// halves take no gap between them: they are one segmented control.
pub fn tool_slots(state: &State) -> Vec<Slot> {
    let mut out = Vec::new();
    let mut x = state.win_w.saturating_sub(PAD_X);
    let sort = sort_text(state);
    for (w, hit, gap) in [
        (pill_w(UNDO_LABEL), HeadHit::Undo, TOOL_GAP),
        (pill_w(&sort), HeadHit::Sort, TOOL_GAP),
        (pill_w(GRID_LABEL), HeadHit::ViewGrid, 0),
        (pill_w(LIST_LABEL), HeadHit::ViewList, TOOL_GAP),
        (SEARCH_W, HeadHit::Search, TOOL_GAP),
    ] {
        x = x.saturating_sub(w);
        out.push(Slot { x, w, hit });
        x = x.saturating_sub(gap);
    }
    out.reverse();
    out
}

/// Left edge the breadcrumb must stop short of, so a deep path never draws under
/// the control strip.
pub fn strip_left(state: &State) -> u32 {
    tool_slots(state).first().map(|s| s.x.saturating_sub(TOOL_GAP)).unwrap_or(state.win_w)
}
