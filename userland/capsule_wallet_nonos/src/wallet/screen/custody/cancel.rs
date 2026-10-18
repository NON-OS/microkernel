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
//! Closing a key screen and wiping what it held.

use nonos_app_skeleton::EventOutcome;

use crate::wallet::state::{State, VIEW_HOME, VIEW_SETTINGS};

/// Where a key screen goes back to: Settings once there is an account,
/// the first screen before.
fn leave(state: &mut State) {
    state.view = if state.acct.address_ready { VIEW_SETTINGS } else { VIEW_HOME };
    state.scroll = 0;
}

/// Close whichever key screen is up, wiping what it held.
pub fn cancel(state: &mut State) -> EventOutcome {
    state.keys.armed = false;
    state.keys.recover_shown = false;
    if state.keys.import_active {
        let _ = crate::wallet::event::toggle_import(state);
    }
    if state.keys.recover_active {
        let _ = crate::wallet::event::toggle_recover(state);
    }
    if state.keys.export_active {
        let _ = crate::wallet::event::toggle_export(state);
    }
    state.failure = None;
    leave(state);
    EventOutcome::Repaint
}
