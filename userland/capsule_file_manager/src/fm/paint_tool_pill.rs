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

use super::header_slots::{Slot, SEARCH_HINT, TOOL_H, TOOL_PAD, TOOL_PX};
use super::state::State;
use super::theme::{CY, INK, INK3, LINE, LINE2, PANEL, RAISE};

// Every slot width came from `pill_w`, which is the measured label plus TOOL_PAD
// on each side, so padding the pen by TOOL_PAD centres the label exactly.
pub fn text_y(y: u32) -> i32 {
    y as i32 + (TOOL_H as i32 - TOOL_PX as i32) / 2 - 2
}

/// One half of the segmented view toggle, drawn onto the shared track.
pub fn half(fb: &mut PaintBuffer, slot: &Slot, y: u32, label: &str, active: bool) {
    if active {
        fb.fill_round(slot.x + 2, y + 2, slot.w.saturating_sub(4), TOOL_H - 4, 7, RAISE);
    }
    let ink = if active { CY } else { INK3 };
    let _ = fb.text_ttf((slot.x + TOOL_PAD) as i32, text_y(y), label, ink, TOOL_PX);
}

/// A standalone labelled control.
pub fn pill(fb: &mut PaintBuffer, slot: &Slot, y: u32, label: &str, ink: u32) {
    fb.panel(slot.x, y, slot.w, TOOL_H, 9, PANEL, LINE);
    let _ = fb.text_ttf((slot.x + TOOL_PAD) as i32, text_y(y), label, ink, TOOL_PX);
}

/// The query field: an emphasised border, and the placeholder in tertiary ink
/// while nothing has been typed.
pub fn search(state: &State, fb: &mut PaintBuffer, slot: &Slot, y: u32) {
    fb.panel(slot.x, y, slot.w, TOOL_H, 9, PANEL, LINE2);
    let empty = state.query.is_empty();
    let text = if empty { SEARCH_HINT } else { state.query.as_str() };
    let ink = if empty { INK3 } else { INK };
    let _ = fb.text_ttf((slot.x + TOOL_PAD) as i32, text_y(y), text, ink, TOOL_PX);
}
