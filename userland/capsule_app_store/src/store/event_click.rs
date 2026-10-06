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

//! The pointer.

use nonos_app_skeleton::EventOutcome;

use super::state::{State, TABS};
use super::ui::chrome::tab_rect;
use super::ui::metrics::{HEAD_H, PAD_TOP, TAB_H};

/// The tab strip first, then the list.
pub fn on_click(state: &mut State, x: i32, y: i32) -> EventOutcome {
    if x < 0 || y < 0 {
        return EventOutcome::Idle;
    }
    let top = (PAD_TOP + HEAD_H) as i32;
    if y < top || y >= top + TAB_H as i32 {
        return super::event_rows::on_list_click(state, x, y);
    }
    match hit(x as u32) {
        Some(i) if state.set_tab(TABS[i]) => EventOutcome::Repaint,
        _ => EventOutcome::Idle,
    }
}

fn hit(x: u32) -> Option<usize> {
    (0..TABS.len()).find(|&i| {
        let (left, w) = tab_rect(i);
        x >= left && x < left + w
    })
}
