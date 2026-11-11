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

use nonos_toolkit::icons::IconId;
use nonos_toolkit::widgets::Button;

use super::header_slots::{
    HeadHit, Slot, FIELD_MIN_W, GLYPH_S, ICON_BTN, NEW_LABEL, SEARCH_W, TOOL_GAP, TOOL_H,
    UNDO_LABEL,
};
use super::layout::{HEADER_H, PAD_X};
use super::state::State;

/// Top of the control band, centred in the header below the window titlebar.
pub fn tool_y() -> u32 {
    (HEADER_H - TOOL_H) / 2 + 6
}

/// The sort control's label: the mode alone, because the sliders glyph beside it
/// already says what the control is.
pub fn sort_text(state: &State) -> String {
    String::from(core::str::from_utf8(state.sort_mode.label()).unwrap_or("?"))
}

/// Width of a labelled control: the shared `Button`'s own, at the strip's em
/// size, so its label is never cut.
pub fn button_w(label: &str) -> u32 {
    Button::new(label).size().0
}

/// Width of a button that leads with a glyph and follows it with a label.
pub fn icon_pill_w(icon: IconId, label: &str) -> u32 {
    Button::new(label).icon(icon).icon_size(GLYPH_S).size().0
}

/// The search control is a full field only where the window is wide enough to
/// leave the breadcrumb a usable share of the band; below that it collapses to
/// its bare glyph. Both the painter and the hit-test read this one width.
pub fn search_w(state: &State) -> u32 {
    if state.win_w >= FIELD_MIN_W {
        SEARCH_W
    } else {
        ICON_BTN
    }
}

/// Whether the search control is currently drawn as a field rather than a glyph.
pub fn search_is_field(state: &State) -> bool {
    search_w(state) > ICON_BTN
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
        (icon_pill_w(IconId::Plus, NEW_LABEL), HeadHit::New, TOOL_GAP),
        (icon_pill_w(IconId::Sliders, &sort), HeadHit::Sort, TOOL_GAP),
        (ICON_BTN, HeadHit::ViewList, 0),
        (ICON_BTN, HeadHit::ViewGrid, TOOL_GAP),
        (search_w(state), HeadHit::Search, TOOL_GAP),
        (button_w(UNDO_LABEL), HeadHit::Undo, TOOL_GAP),
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
