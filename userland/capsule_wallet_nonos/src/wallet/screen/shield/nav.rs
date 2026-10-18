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

/*
 * Into the Shield and back out: back walks one step, from review to the
 * screen it was opened from, from any other Shield screen to Shield home,
 * and from Shield home to the wallet.
 */

use nonos_app_skeleton::EventOutcome;

use crate::wallet::state::shield_ui::{SHIELD_HOME, SHIELD_PROVING, SHIELD_REVIEW, SHIELD_SETTLE};
use crate::wallet::state::{State, VIEW_HOME, VIEW_SHIELD};

/* Enter Shield from the wallet: ask whether a shield service is there and,
 * if it is, open this account's shield store in it. */
pub fn open(state: &mut State) -> EventOutcome {
    state.shield_ui.screen = SHIELD_HOME;
    state.shield_ui.absent_hidden = false;
    state.shield_ui.failure = None;
    state.failure = None;
    if crate::wallet::shield::open::here() {
        state.shield = crate::wallet::shield::probe::probe();
    }
    if crate::wallet::shield::open::here() && state.shield.available() {
        crate::wallet::shield::open::open(state);
    }
    state.view = VIEW_SHIELD;
    state.scroll = 0;
    EventOutcome::Repaint
}

pub fn back(state: &mut State) -> EventOutcome {
    let ui = &mut state.shield_ui;
    ui.failure = None;
    // An escrow deposit is never left filled in for a later withdrawal:
    // Fund sets it again when the person comes back to it (B8).
    if ui.payee.to_deposit.take().is_some() {
        ui.amount.clear();
    }
    match ui.screen {
        SHIELD_HOME => state.view = VIEW_HOME,
        SHIELD_REVIEW if ui.from == SHIELD_SETTLE => ui.screen = SHIELD_PROVING,
        SHIELD_REVIEW => ui.screen = ui.from,
        _ => ui.screen = SHIELD_HOME,
    }
    state.scroll = 0;
    EventOutcome::Repaint
}
