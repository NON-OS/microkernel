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

pub fn open(state: &mut State) -> EventOutcome {
    state.view = crate::wallet::state::VIEW_SEND;
    state.send_stage = STAGE_FORM;
    state.send_draft = None;
    state.failure = None;
    state.scroll = 0;
    EventOutcome::Repaint
}

pub fn back(state: &mut State) -> EventOutcome {
    /* A payment going out ends on the Sent screen, never back on the form,
     * where it could be paid again before its outcome is known. */
    if crate::wallet::act::running(state) {
        return EventOutcome::Idle;
    }
    state.failure = None;
    state.scroll = 0;
    match state.send_stage {
        STAGE_REVIEW => {
            state.send_draft = None;
            state.send_stage = STAGE_FORM;
        }
        _ => {
            state.send_stage = STAGE_FORM;
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
fn confirm(state: &mut State) -> EventOutcome {
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

pub fn click(state: &mut State, press: Press) -> EventOutcome {
    /* While the network is read or a payment goes out, the screen holds:
     * only a shown reason may be put away. */
    if crate::wallet::act::running(state) && press != Press::Dismiss {
        return EventOutcome::Idle;
    }
    match (press, state.send_stage) {
        (Press::Back, _) => return back(state),
        (Press::Dismiss, _) => state.failure = None,
        (Press::Asset(a), STAGE_FORM) if a <= send::ASSET_USDC => {
            if a != state.send_token {
                state.send_token = a;
                state.send_amount = crate::wallet::num::Amount::new();
                state.send_all = false;
            }
        }
        (Press::Field(f), STAGE_FORM) => state.send_focus = f,
        (Press::Footer(0), STAGE_FORM) => return review(state),
        (Press::Footer(1), STAGE_FORM) => use_all(state),
        (Press::Footer(0), STAGE_REVIEW) => return confirm(state),
        (Press::Footer(1), STAGE_REVIEW) => return back(state),
        (Press::Footer(0), _) => {
            state.send_stage = STAGE_FORM;
            state.view = VIEW_HOME;
            state.scroll = 0;
        }
        _ => return EventOutcome::Idle,
    }
    EventOutcome::Repaint
}

/* Everything held, in the field. For ETH the field shows what is held less
 * a fee at the last fee read; the review sets the exact amount aside from
 * the fees it reads itself. Typing in the field undoes it. */
fn use_all(state: &mut State) {
    let asset = state.send_token;
    let Some(have) = send::held(state, asset) else {
        state.failure = Some("The balance has not been read yet. Try again shortly.");
        return;
    };
    let shown = if asset == send::ASSET_ETH {
        let rough = u128::from(state.fee_wei).saturating_mul(2);
        have.saturating_sub(rough.saturating_mul(u128::from(send::gas::PLAIN_GAS)))
    } else {
        have
    };
    state.send_amount = crate::wallet::num::Amount::of_units(shown, send::decimals(asset));
    state.send_all = true;
    state.send_focus = crate::wallet::state::SEND_FIELD_AMOUNT;
    state.failure = None;
}
