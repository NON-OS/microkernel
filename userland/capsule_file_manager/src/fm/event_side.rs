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

use alloc::vec::Vec;

use nonos_app_skeleton::EventOutcome;

use super::navigate::navigate;
use super::sidebar_model::{SideHit, SideRow};
use super::sidebar_rows::side_rows;
use super::state::State;

/// The sidebar's rows for the state on show, as `paint_sidebar` draws them.
pub fn shown_rows(state: &State) -> Vec<SideRow> {
    side_rows(&state.favorites.list(), state.win_h)
}

/// Route a sidebar click through the one layout pass `paint_sidebar` drew from.
/// `index` is the row the target table named; a section label carries no hit
/// and is never named, which is why a miss is `Idle` rather than a fall-through
/// to the listing underneath.
pub fn on_side(state: &mut State, index: usize) -> EventOutcome {
    match shown_rows(state).into_iter().nth(index).and_then(|r| r.hit) {
        Some(SideHit::Screen(screen)) => {
            state.screen = screen;
            EventOutcome::Repaint
        }
        Some(SideHit::Path(path)) => {
            navigate(state, path.as_str());
            EventOutcome::Repaint
        }
        None => EventOutcome::Idle,
    }
}
