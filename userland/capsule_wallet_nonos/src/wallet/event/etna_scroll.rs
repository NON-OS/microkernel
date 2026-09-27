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

//! Scrolling an Etna screen.

use nonos_app_skeleton::EventOutcome;

use super::etna_click::on_etna;
use crate::wallet::screen::hits::limit;
use crate::wallet::state::State;

/// The wheel moves the photograph and content under the fixed bar and foot,
/// no further than the screen's content reaches.
pub fn scroll(state: &mut State, delta_y: i32) -> EventOutcome {
    if !on_etna(state) {
        return EventOutcome::Idle;
    }
    let next = (i64::from(state.scroll) - i64::from(delta_y) * 60).clamp(0, i64::from(limit()));
    state.scroll = next as u32;
    EventOutcome::Repaint
}
