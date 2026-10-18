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
//! Opening, confirming and leaving a staking review.

use nonos_app_skeleton::EventOutcome;

use crate::wallet::act::Purpose;
use crate::wallet::state::{State, VIEW_HOME};

/// Read the network for the staking transaction asked for next.
pub fn review(state: &mut State) -> EventOutcome {
    if crate::wallet::act::running(state) || state.stake.draft.is_some() {
        return EventOutcome::Idle;
    }
    if let Err(why) = crate::wallet::act::review_stake(state) {
        state.failure = Some(why);
    }
    EventOutcome::Repaint
}

/// Sign the reviewed staking transaction and send it, once.
pub(super) fn confirm(state: &mut State) -> EventOutcome {
    let Some(draft) = state.stake.draft.clone() else {
        return EventOutcome::Idle;
    };
    let kind: &'static [u8] = match draft.purpose {
        Purpose::Approve => b"APPROVE",
        Purpose::Unstake => b"UNSTAKE",
        _ => b"STAKE",
    };
    match crate::wallet::send::sign_draft(state, &draft, kind) {
        Ok(()) => {
            state.failure = None;
            crate::wallet::act::broadcast(state, draft.purpose);
        }
        Err(why) => {
            state.stake.draft = None;
            state.failure = Some(why);
        }
    }
    EventOutcome::Repaint
}

/// Back from a review drops it; back from the form leaves the screen.
pub fn back(state: &mut State) -> EventOutcome {
    if crate::wallet::act::running(state) {
        return EventOutcome::Idle;
    }
    state.failure = None;
    state.scroll = 0;
    if state.stake.draft.take().is_none() {
        state.view = VIEW_HOME;
    }
    EventOutcome::Repaint
}
