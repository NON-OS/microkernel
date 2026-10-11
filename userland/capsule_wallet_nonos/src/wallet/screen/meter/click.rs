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

use crate::wallet::screen::hits::Press;
use crate::wallet::state::{State, VIEW_METER, VIEW_SETTINGS};

/// Open the meter screen, asking the meter how it stands.
pub fn open(state: &mut State) -> EventOutcome {
    state.view = VIEW_METER;
    state.scroll = 0;
    refresh(state)
}

fn refresh(state: &mut State) -> EventOutcome {
    state.meter.account = crate::wallet::shield::meter_call::account();
    state.meter.asked = true;
    EventOutcome::Repaint
}

pub fn click(state: &mut State, press: Press) -> EventOutcome {
    match press {
        Press::Back => state.view = VIEW_SETTINGS,
        Press::Dismiss => state.failure = None,
        Press::Footer(0) => return refresh(state),
        _ => return EventOutcome::Idle,
    }
    state.scroll = 0;
    EventOutcome::Repaint
}
