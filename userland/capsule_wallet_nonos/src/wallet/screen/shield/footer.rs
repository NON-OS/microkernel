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
 * The footer buttons of the Shield screens. The three choosing screens ask
 * the service for a review; the review confirms or goes back to edit; the
 * proving screen stops a proof, settles a kept payment from this account or
 * takes its notes back; the network page copies the faucet's address.
 */

use nonos_app_skeleton::clients::clipboard::clipboard_copy;
use nonos_app_skeleton::EventOutcome;

use super::check::{deposit_ready, send_ready, withdraw_ready};
use crate::wallet::shield::{actions, job};
use crate::wallet::state::shield_ui::*;
use crate::wallet::state::State;

pub fn footer(state: &mut State, n: u8) -> EventOutcome {
    if !crate::wallet::shield::open::here() {
        return elsewhere(state, n);
    }
    let screen = state.shield_ui.screen;
    state.shield_ui.news = None;
    match (screen, n) {
        (SHIELD_DEPOSIT, 0) if deposit_ready(&state.shield_ui) => actions::review_deposit(state),
        (SHIELD_WITHDRAW, 0) if withdraw_ready(&state.shield_ui) => actions::review_withdraw(state),
        (SHIELD_SEND, 0) if send_ready(&state.shield_ui) => actions::review_send(state),
        (SHIELD_REVIEW, 0) if super::absent::ready(state) => actions::confirm(state),
        (SHIELD_REVIEW, 1) => return super::click::back(state),
        (SHIELD_PROVING, _) => {
            let shown = super::proving::buttons(&state.shield_ui);
            match shown.get(n as usize).map(|(_, b)| *b) {
                Some(super::proving::STOP) => job::cancel(),
                Some(super::proving::SETTLE) => actions::review_self_settle(state, None),
                Some(super::proving::SETTLE_EARLIER) => {
                    let id = super::proving::earlier_to_settle(&state.shield_ui)
                        .map(alloc::string::String::from);
                    actions::review_self_settle(state, id)
                }
                Some(super::proving::TAKE_BACK) => actions::take_back(state),
                _ => return EventOutcome::Idle,
            }
        }
        (SHIELD_NETWORK, 0) => {
            state.status = match clipboard_copy(super::consts::FAUCET.as_bytes()) {
                Ok(()) => b"faucet address copied",
                Err(_) => b"the clipboard did not take it",
            };
        }
        _ => return EventOutcome::Idle,
    }
    EventOutcome::Repaint
}

/* The one press off the shield's network: go to Sepolia and open it. */
fn elsewhere(state: &mut State, n: u8) -> EventOutcome {
    if n != 0 {
        return EventOutcome::Idle;
    }
    match crate::wallet::state::switch_network(state, true) {
        Ok(_) => {
            state.status = b"network changed";
            super::click::open(state)
        }
        Err(why) => {
            state.failure = Some(why);
            EventOutcome::Repaint
        }
    }
}
