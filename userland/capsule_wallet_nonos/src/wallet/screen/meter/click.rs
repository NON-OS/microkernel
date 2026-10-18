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

use super::channels::{FIELD_TOPUP, ROW_CLOSE, ROW_TOPUP};
use super::claim::ROW_CLAIM;
use crate::wallet::screen::hits::Press;
use crate::wallet::state::{State, VIEW_METER, VIEW_SETTINGS};

/// Open the meter screen, asking the meter how it stands.
pub fn open(state: &mut State) -> EventOutcome {
    state.view = VIEW_METER;
    state.scroll = 0;
    refresh(state)
}

pub(super) fn refresh(state: &mut State) -> EventOutcome {
    state.meter.account = crate::wallet::meter::call::account(0);
    state.meter.asked = true;
    EventOutcome::Repaint
}

pub fn click(state: &mut State, press: Press) -> EventOutcome {
    match press {
        Press::Back => state.view = VIEW_SETTINGS,
        Press::Dismiss => state.failure = None,
        Press::Footer(0) => return refresh(state),
        Press::Footer(1) => return super::budget::set(state),
        Press::Field(f) => state.meter.focus = f,
        Press::Row(ROW_TOPUP) => return super::jobs::topup(state),
        Press::Row(ROW_CLOSE) => return super::jobs::close(state),
        Press::Row(ROW_CLAIM) => return super::claim::claim(state),
        Press::Row(i) => {
            state.meter.picked = Some(i);
            state.meter.focus = FIELD_TOPUP;
        }
        _ => return EventOutcome::Idle,
    }
    state.scroll = 0;
    EventOutcome::Repaint
}
