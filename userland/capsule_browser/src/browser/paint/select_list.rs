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

//! An open select's list, drawn over the page as a shared Menu
//! (ui::select_menu places and draws it, event::select_list holds it).

use nonos_app_skeleton::PaintBuffer;

use crate::browser::event::select_placed;
use crate::browser::state::State;
use crate::browser::ui::select_menu::draw;

/// Draw the open list, if there is one, over whatever the page drew.
pub fn paint(state: &State, fb: &mut PaintBuffer) {
    if let (Some(list), Some(at)) = (state.ui.select.as_ref(), select_placed(state)) {
        draw(list, fb, at);
    }
}
