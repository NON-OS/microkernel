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
//! Keys on the staking screen.

use nonos_app_skeleton::EventOutcome;

use super::confirm::review;
use crate::wallet::state::{State, VIEW_NOX};

pub fn key(state: &mut State, code: u32) -> Option<EventOutcome> {
    if state.view != VIEW_NOX {
        return None;
    }
    /* Enter opens the review, never more: a review is confirmed by its button. */
    if code == nonos_app_skeleton::KEY_ENTER {
        return Some(review(state));
    }
    if state.stake.draft.is_some() || crate::wallet::act::running(state) {
        return Some(EventOutcome::Idle);
    }
    state.failure = None;
    crate::wallet::event::stake_input(state, code)
}
