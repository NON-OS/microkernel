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
use nonos_toolkit::widgets::{Button, Rect, State as Look};

use super::header_bar::clear_slot;
use super::header_layout::{sort_text, tool_slots, tool_y};
use super::header_nav::{can_back, can_fwd, nav_slots};
use super::header_slots::{strip, HeadHit, Slot, UNDO_LABEL};
use super::icon_path::Icon;
use super::state::{State, ViewKind};
use super::target::Target;
use super::target_at::look;
use super::toolbar_buttons::{glyph_btn, group_plate, icon_btn};
use super::toolbar_new::{icon_label, new_btn};
use super::toolbar_search::search;

pub fn paint_toolbar(state: &State, fb: &mut PaintBuffer) {
    let y = tool_y();
    paint_nav(state, fb, y);
    let slots = tool_slots(state);
    toggle_ground(fb, &slots, y);
    let sort = sort_text(state);
    let clear = clear_slot(state, &slots);
    for slot in &slots {
        match slot.hit {
            HeadHit::Search => search(state, fb, slot, y, clear.as_ref()),
            HeadHit::ViewGrid => {
                icon_btn(state, fb, slot, y, Icon::Grid, state.view == ViewKind::Grid)
            }
            HeadHit::ViewList => {
                icon_btn(state, fb, slot, y, Icon::List, state.view == ViewKind::List)
            }
            HeadHit::Sort => icon_label(state, fb, slot, y, IconId::Sliders, &sort),
            HeadHit::Undo => undo(state, fb, slot, y),
            HeadHit::New => new_btn(state, fb, slot, y),
            _ => {}
        }
    }
}

// The history group. Forward has no visit stack behind it, so it draws dimmed
// and stays inert rather than looking live and doing nothing.
fn paint_nav(state: &State, fb: &mut PaintBuffer, y: u32) {
    let slots = nav_slots();
    let (Some(first), Some(last)) = (slots.first(), slots.last()) else { return };
    group_plate(fb, first, last, y);
    for slot in &slots {
        let back = slot.hit == HeadHit::NavBack;
        let live = if back { can_back(state) } else { can_fwd(state) };
        let icon = if back { IconId::Back } else { IconId::Forward };
        glyph_btn(state, fb, slot, y, icon, !live);
    }
}

// Undo draws disabled when the stack holds nothing to replay.
fn undo(state: &State, fb: &mut PaintBuffer, slot: &Slot, y: u32) {
    let lit = Look { disabled: state.undo.is_empty(), ..look(state, Target::Head(slot.hit)) };
    Button::new(UNDO_LABEL).state(lit).draw(fb, Rect { x: slot.x, y, w: slot.w, h: strip().h });
}

// The two halves share one ground, so the toggle reads as a single track with a
// lit active half rather than as two adjacent pills.
fn toggle_ground(fb: &mut PaintBuffer, slots: &[Slot], y: u32) {
    let grid = slots.iter().find(|s| s.hit == HeadHit::ViewGrid);
    let list = slots.iter().find(|s| s.hit == HeadHit::ViewList);
    if let (Some(grid), Some(list)) = (grid, list) {
        group_plate(fb, grid, list, y);
    }
}
