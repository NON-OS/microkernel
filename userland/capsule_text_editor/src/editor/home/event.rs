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

//! Input routing for the Home screen: the pointer is fed to the screen's
//! controls, which act on press as the screen always has.

use nonos_app_skeleton::{feed, EventOutcome, InputEvent};
use nonos_toolkit::widgets::Step;

use super::super::app::Editor;
use super::doc_click::doc_click;
use super::pick::HomePick;
use super::state::HomeState;

pub(crate) fn home_event(ed: &mut Editor, event: &InputEvent) -> EventOutcome {
    match feed(&mut ed.home_ctl, event) {
        Step::Act(HomePick::Nav(row)) => {
            HomeState::select(row);
        }
        Step::Act(HomePick::ViewAll) => {
            HomeState::select(0);
        }
        Step::Act(HomePick::Doc(i)) => doc_click(ed, i),
        Step::Repaint => {}
        Step::Idle => return EventOutcome::Idle,
    }
    EventOutcome::Repaint
}
