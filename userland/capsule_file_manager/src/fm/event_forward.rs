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

use nonos_app_skeleton::EventOutcome;

use super::refresh::refresh;
use super::screen::Screen;
use super::state::State;

/// Forward: back down into the folder Back last went up out of
/// (`nav_trail.rs`). A folder gone since says so through the listing.
pub fn open_forward(state: &mut State) -> EventOutcome {
    let Some(dir) = state.trail.forward() else { return EventOutcome::Idle };
    state.screen = Screen::Browse;
    state.prefix = dir;
    state.cursor = 0;
    state.scroll = 0;
    refresh(state);
    EventOutcome::Repaint
}
