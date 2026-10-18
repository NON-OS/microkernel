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
//! Stepping between the payment screens: back, on to the review, and the
//! confirm that signs and sends.

use nonos_app_skeleton::EventOutcome;

use crate::wallet::send::{self, STAGE_FORM, STAGE_REVIEW};
use crate::wallet::state::{State, VIEW_HOME};

pub fn back(state: &mut State) -> EventOutcome {
    /* A payment going out ends on the Sent screen, never back on the form,
     * where it could be paid again before its outcome is known. */
    if crate::wallet::act::running(state) {
        return EventOutcome::Idle;
    }
    state.failure = None;
    state.scroll = 0;
    match state.send.stage {
        STAGE_REVIEW => {
            state.send.draft = None;
            state.send.stage = STAGE_FORM;
        }
        _ => {
            state.send.stage = STAGE_FORM;
            state.view = VIEW_HOME;
        }
    }
    EventOutcome::Repaint
}

/// Work out the payment and begin reading the network for its review, or
/// say why not. The review shows once the reads are in (`act::review`).
pub fn review(state: &mut State) -> EventOutcome {
    if crate::wallet::act::running(state) {
        return EventOutcome::Idle;
    }
    if let Err(why) = crate::wallet::act::review(state) {
        state.failure = Some(why);
    }
    EventOutcome::Repaint
}

/// Sign the reviewed payment and begin sending it, once. What the network
/// said shows when it ends (`act::broadcast`).
pub(super) fn confirm(state: &mut State) -> EventOutcome {
    if crate::wallet::act::running(state) {
        return EventOutcome::Idle;
    }
    match send::sign(state) {
        Ok(()) => {
            state.failure = None;
            crate::wallet::act::broadcast(state, crate::wallet::act::Purpose::Pay);
        }
        Err(why) => state.failure = Some(why),
    }
    state.scroll = 0;
    EventOutcome::Repaint
}
