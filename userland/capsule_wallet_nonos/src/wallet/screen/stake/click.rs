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

use crate::wallet::screen::hits::Press;
use crate::wallet::state::State;

use super::confirm::confirm;
pub use super::confirm::{back, review};
pub use super::key::key;

const UNREAD_BALANCE: &str =
    "The NOX balance has not been read yet, so there is no whole balance to fill in.";

pub fn click(state: &mut State, press: Press) -> EventOutcome {
    /* While the network is read or a transaction goes out, the screen holds. */
    if crate::wallet::act::running(state) && press != Press::Dismiss {
        return EventOutcome::Idle;
    }
    if state.stake.draft.is_some() {
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
        Press::Tab(t) => state.stake.unstake = t.min(1),
        Press::Term(t) => state.stake.lock = t,
        Press::Pick(p) => state.stake.position = u64::from(p),
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
