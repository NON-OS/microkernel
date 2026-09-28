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
 * The footer buttons of the Shield screens. The three choosing screens
 * open the review; the review confirms or goes back to edit; the network
 * page copies the faucet's address.
 */

use nonos_app_skeleton::clients::clipboard::clipboard_copy;
use nonos_app_skeleton::EventOutcome;

use crate::wallet::state::shield_ui::*;
use crate::wallet::state::State;

pub fn footer(state: &mut State, n: u8) -> EventOutcome {
    let screen = state.shield_ui.screen;
    match (screen, n) {
        (SHIELD_DEPOSIT, 0) if state.shield_ui.size.is_some() => open(state),
        (SHIELD_WITHDRAW, 0) if super::check::withdraw_ready(&state.shield_ui) => open(state),
        (SHIELD_SEND, 0) if super::check::send_ready(&state.shield_ui) => open(state),
        (SHIELD_REVIEW, 0) => {
            if let Err(why) = super::request::submit(state) {
                state.shield_ui.failure = Some(why);
            }
            EventOutcome::Repaint
        }
        (SHIELD_REVIEW, 1) => super::click::back(state),
        (SHIELD_NETWORK, 0) => {
            let _ = clipboard_copy(super::consts::FAUCET.as_bytes());
            state.status = b"faucet address copied";
            EventOutcome::Repaint
        }
        _ => EventOutcome::Idle,
    }
}

fn open(state: &mut State) -> EventOutcome {
    let ui = &mut state.shield_ui;
    ui.from = ui.screen;
    ui.screen = SHIELD_REVIEW;
    ui.failure = None;
    state.scroll = 0;
    EventOutcome::Repaint
}
