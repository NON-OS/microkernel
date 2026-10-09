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

//! Esc goes back one step, as the back control in the bar does: a payment
//! from its review to its form, a Shield screen toward Shield home, any
//! other screen to the wallet, and the wallet stays where it is.

use nonos_app_skeleton::EventOutcome;

use crate::wallet::screen;
use crate::wallet::state::{State, VIEW_HOME, VIEW_NOX, VIEW_SEND, VIEW_SHIELD};

pub fn escape(state: &mut State) -> EventOutcome {
    match state.view {
        VIEW_SHIELD => screen::shield::click::back(state),
        VIEW_SEND => screen::pay::click::back(state),
        VIEW_NOX => screen::stake::click::back(state),
        VIEW_HOME => EventOutcome::Idle,
        _ => {
            state.view = VIEW_HOME;
            state.scroll = 0;
            state.failure = None;
            EventOutcome::Repaint
        }
    }
}
