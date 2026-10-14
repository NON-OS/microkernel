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

//! Where a review's refusal is said.

use super::Purpose;
use crate::wallet::send::STAGE_FORM;
use crate::wallet::state::{State, VIEW_NOX, VIEW_SEND};

/* Whether the screen the review was asked from is still up, unreviewed. */
pub(super) fn asked_from(state: &State, purpose: Purpose) -> bool {
    match purpose {
        Purpose::Pay => state.view == VIEW_SEND && state.send.stage == STAGE_FORM,
        _ => state.view == VIEW_NOX && state.stake.draft.is_none(),
    }
}

/* Said on the screen it was asked from, and nowhere else. */
pub(super) fn refuse(state: &mut State, why: &'static str) {
    let on_form = state.view == VIEW_SEND && state.send.stage == STAGE_FORM;
    let on_stake = state.view == VIEW_NOX && state.stake.draft.is_none();
    if on_form || on_stake {
        state.failure = Some(why);
    }
}
