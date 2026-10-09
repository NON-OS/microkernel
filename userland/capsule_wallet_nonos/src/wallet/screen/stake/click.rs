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
//! Presses and keys on the staking screen. The button and Enter only open
//! a review, made from fresh reads (`act::review_stake`); only the review's
//! Confirm signs, and then the transaction goes out once.

use nonos_app_skeleton::EventOutcome;

use crate::wallet::act::Purpose;
use crate::wallet::screen::hits::Press;
use crate::wallet::state::{State, VIEW_HOME, VIEW_NOX};

const UNREAD_BALANCE: &str =
    "The NOX balance has not been read yet, so there is no whole balance to fill in.";

/// Read the network for the staking transaction asked for next.
pub fn review(state: &mut State) -> EventOutcome {
    if crate::wallet::act::running(state) || state.stake_draft.is_some() {
        return EventOutcome::Idle;
    }
    if let Err(why) = crate::wallet::act::review_stake(state) {
        state.failure = Some(why);
    }
    EventOutcome::Repaint
}

/// Sign the reviewed staking transaction and send it, once.
fn confirm(state: &mut State) -> EventOutcome {
    let Some(draft) = state.stake_draft.clone() else {
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
            state.stake_draft = None;
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
    if state.stake_draft.take().is_none() {
        state.view = VIEW_HOME;
    }
    EventOutcome::Repaint
}

pub fn click(state: &mut State, press: Press) -> EventOutcome {
    /* While the network is read or a transaction goes out, the screen holds. */
    if crate::wallet::act::running(state) && press != Press::Dismiss {
        return EventOutcome::Idle;
    }
    if state.stake_draft.is_some() {
        return match press {
            Press::Footer(0) => confirm(state),
            Press::Footer(1) | Press::Back => back(state),
            Press::Dismiss => {
                state.failure = None;
                EventOutcome::Repaint
            }
            _ => EventOutcome::Idle,
        };
    }
    match press {
        Press::Back => return back(state),
        Press::Dismiss => state.failure = None,
        Press::Tab(t) => state.stake_unstake = t.min(1),
        Press::Term(t) => state.stake_lock = t,
        Press::Pick(p) => state.stake_position = u64::from(p),
        Press::Field(_) => {}
        Press::Footer(0) => return review(state),
        Press::Footer(1) => {
            if !crate::wallet::event::stake_set_max(state) {
                state.failure = Some(UNREAD_BALANCE);
            }
        }
        _ => return EventOutcome::Idle,
    }
    EventOutcome::Repaint
}

pub fn key(state: &mut State, code: u32) -> Option<EventOutcome> {
    if state.view != VIEW_NOX {
        return None;
    }
    /* Enter opens the review, never more: a review is confirmed by its button. */
    if code == nonos_app_skeleton::KEY_ENTER {
        return Some(review(state));
    }
    if state.stake_draft.is_some() || crate::wallet::act::running(state) {
        return Some(EventOutcome::Idle);
    }
    state.failure = None;
    crate::wallet::event::stake_input(state, code)
}
