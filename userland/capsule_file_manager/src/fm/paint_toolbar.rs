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

use super::header_layout::{sort_text, tool_slots, tool_y};
use super::header_slots::{HeadHit, Slot, GRID_LABEL, LIST_LABEL, TOOL_H, UNDO_LABEL};
use super::paint_tool_pill::{half, pill, search};
use super::state::{State, ViewKind};
use super::theme::{INK, INK2, INK3, LINE, PANEL};

pub fn paint_toolbar(state: &State, fb: &mut PaintBuffer) {
    let y = tool_y();
    let slots = tool_slots(state);
    toggle_ground(fb, &slots, y);
    let sort = sort_text(state);
    for slot in &slots {
        match slot.hit {
            HeadHit::Search => search(state, fb, slot, y),
            HeadHit::ViewList => half(fb, slot, y, LIST_LABEL, state.view == ViewKind::List),
            HeadHit::ViewGrid => half(fb, slot, y, GRID_LABEL, state.view == ViewKind::Grid),
            HeadHit::Sort => pill(fb, slot, y, &sort, INK2),
            HeadHit::Undo => pill(fb, slot, y, UNDO_LABEL, undo_ink(state)),
            HeadHit::Crumb(_) => {}
        }
    }
}

// Undo dims to the tertiary ink when the stack holds nothing to replay.
fn undo_ink(state: &State) -> u32 {
    if state.undo.is_empty() {
        INK3
    } else {
        INK
    }
}

// The two halves share one panel, so the toggle reads as a single track with a
// raised active half rather than as two adjacent pills.
fn toggle_ground(fb: &mut PaintBuffer, slots: &[Slot], y: u32) {
    let list = slots.iter().find(|s| s.hit == HeadHit::ViewList);
    let grid = slots.iter().find(|s| s.hit == HeadHit::ViewGrid);
    if let (Some(list), Some(grid)) = (list, grid) {
        fb.panel(list.x, y, (grid.x + grid.w).saturating_sub(list.x), TOOL_H, 9, PANEL, LINE);
    }
}
