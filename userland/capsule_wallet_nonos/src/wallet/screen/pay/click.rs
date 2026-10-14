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
//! A press on the payment screens.

use nonos_app_skeleton::EventOutcome;

use crate::wallet::screen::hits::Press;
use crate::wallet::send::{self, STAGE_FORM, STAGE_REVIEW};
use crate::wallet::state::{State, VIEW_HOME};

use super::click_all::use_all;
use super::click_step::confirm;
pub use super::click_step::{back, review};

pub fn open(state: &mut State) -> EventOutcome {
    state.view = crate::wallet::state::VIEW_SEND;
    state.send.stage = STAGE_FORM;
    state.send.draft = None;
    state.failure = None;
    state.scroll = 0;
    EventOutcome::Repaint
}

pub fn click(state: &mut State, press: Press) -> EventOutcome {
    /* While the network is read or a payment goes out, the screen holds:
     * only a shown reason may be put away. */
    if crate::wallet::act::running(state) && press != Press::Dismiss {
        return EventOutcome::Idle;
    }
    match (press, state.send.stage) {
        (Press::Back, _) => return back(state),
        (Press::Dismiss, _) => state.failure = None,
        (Press::Asset(a), STAGE_FORM) if a <= send::ASSET_USDC => {
            if a != state.send.token {
                state.send.token = a;
                state.send.amount = crate::wallet::num::Amount::new();
                state.send.all = false;
            }
        }
        (Press::Field(f), STAGE_FORM) => state.send.focus = f,
        (Press::Footer(0), STAGE_FORM) => return review(state),
        (Press::Footer(1), STAGE_FORM) => use_all(state),
        (Press::Footer(0), STAGE_REVIEW) => return confirm(state),
        (Press::Footer(1), STAGE_REVIEW) => return back(state),
        (Press::Footer(0), _) => {
            state.send.stage = STAGE_FORM;
            state.view = VIEW_HOME;
            state.scroll = 0;
        }
        _ => return EventOutcome::Idle,
    }
    EventOutcome::Repaint
}
