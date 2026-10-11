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

//! Buy starts the order; Close drops the purchase, never while a job runs.

use nonos_app_skeleton::EventOutcome;

use super::fund::funding;
use super::words::closable;
use crate::wallet::screen::hits::Press;
use crate::wallet::state::{State, VIEW_HOME};

pub fn click(state: &mut State, press: Press) -> EventOutcome {
    match press {
        Press::Dismiss => state.failure = None,
        Press::Footer(1) if funding(state) => return super::fund::fund(state),
        Press::Footer(1) if opening(state) => return open_channel(state),
        Press::Field(f) => state.meter.purchase.iter_mut().for_each(|p| p.form.focus = f),
        Press::Footer(1) => return super::buy::confirm(state),
        Press::Footer(0) if state.meter.purchase.as_deref().is_some_and(closable) => {
            state.meter.purchase = None;
            state.view = VIEW_HOME;
        }
        _ => return EventOutcome::Idle,
    }
    EventOutcome::Repaint
}

fn opening(state: &State) -> bool {
    state.meter.purchase.as_deref().is_some_and(super::form::open_form)
}

/// Open channel: OPEN START with the form as typed, or why not.
pub fn open_channel(state: &mut State) -> EventOutcome {
    state.failure = crate::wallet::order::opening::open(state);
    EventOutcome::Repaint
}
