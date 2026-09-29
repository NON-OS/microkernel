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

//! Clicking a card.

use nonos_app_skeleton::EventOutcome;

use super::install;
use super::state::State;
use super::ui::geometry::{list_top, list_w, on_action, row_top, slot_at};
use super::ui::metrics::PAD_X;

pub fn on_list_click(state: &mut State, x: i32, y: i32) -> EventOutcome {
    if y < list_top() as i32 || x < PAD_X as i32 {
        return EventOutcome::Idle;
    }
    let width = list_w(state.fb_w);
    if x >= (PAD_X + width) as i32 {
        return EventOutcome::Idle;
    }
    let Some(slot) = slot_at(y, state.rows) else {
        return EventOutcome::Idle;
    };
    if state.scroll + slot >= state.visible().len() {
        return EventOutcome::Idle;
    }
    let moved = state.select_slot(slot);
    if on_action(x, y, PAD_X, row_top(slot), width) {
        state.asked = Some(install::ask(state));
        return EventOutcome::Repaint;
    }
    match moved {
        true => EventOutcome::Repaint,
        false => EventOutcome::Idle,
    }
}
