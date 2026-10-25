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

//! The service's state read again after a job, and a job that failed.

use alloc::format;
use alloc::string::String;

use shield_wire::*;

use crate::wallet::state::shield_ui::*;
use crate::wallet::state::State;

use super::apply_state::state as service_state;

/* The service's state read again after a job. One that did not answer is
 * said, and what was shown stays until the next read. */
pub(super) fn refresh(state: &mut State) {
    match super::client::call(OP_STATE, &[]) {
        Ok(a) if a.status == STATUS_OK => service_state(state, &a.body),
        Ok(a) => {
            let why = field(&a.body, "why").unwrap_or(super::client::NO_ANSWER);
            state.shield_ui.failure =
                Some(format!("The shield's state could not be read again: {why}"));
        }
        Err(why) => {
            state.shield_ui.failure =
                Some(format!("The shield's state could not be read again: {why}"));
        }
    }
}
pub fn failed(state: &mut State, op: u16, why: String) {
    let ui = &mut state.shield_ui;
    if matches!(op, OP_OPEN_WORDS | OP_OPEN_KEY) {
        ui.opened = false;
    }
    if op == OP_WITHDRAW {
        crate::wallet::order::gas_paid::failed(state);
    }
    let ui = &mut state.shield_ui;
    if matches!(op, OP_SEND | OP_WITHDRAW) {
        ui.proving = None;
        if ui.screen == SHIELD_PROVING {
            ui.screen = ui.from;
        }
    }
    ui.failure = Some(why);
}
