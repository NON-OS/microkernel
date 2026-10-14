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

//! A press, sent to the screen it was drawn on.

use nonos_app_skeleton::EventOutcome;

use super::etna_click_receive::receive;
use super::etna_click_views::{home, welcome};
use crate::wallet::screen;
use crate::wallet::screen::hits::{at, Press};
use crate::wallet::state::{
    State, VIEW_ACCOUNTS, VIEW_EXPORT, VIEW_HOME, VIEW_IMPORT, VIEW_METER, VIEW_NOX,
    VIEW_PUBLISHER, VIEW_PURCHASE, VIEW_RECEIVE, VIEW_RECOVER, VIEW_SEND, VIEW_SETTINGS,
    VIEW_SHIELD, VIEW_SWAP,
};

pub(super) fn go(state: &mut State, view: u8) -> EventOutcome {
    state.view = view;
    state.scroll = 0;
    state.failure = None;
    EventOutcome::Repaint
}

pub fn etna_click(state: &mut State, x: u32, y: u32) -> EventOutcome {
    let Some(press) = at(x, y) else {
        return EventOutcome::Idle;
    };
    if state.locked {
        if press == Press::Footer(0) {
            return super::locking::lock::unlock(state);
        }
        return EventOutcome::Idle;
    }
    if state.backup_active {
        if press != Press::Footer(0) {
            return EventOutcome::Idle;
        }
        let _ = super::custody::backup::confirm_backup(state);
        return go(state, VIEW_HOME);
    }
    match state.view {
        VIEW_IMPORT | VIEW_RECOVER | VIEW_EXPORT => screen::custody::click::click(state, press),
        VIEW_SETTINGS => screen::settings::click::click(state, press),
        VIEW_ACCOUNTS if state.address_ready => screen::accounts::click::click(state, press),
        _ if !state.address_ready => welcome(state, press),
        VIEW_SHIELD => screen::shield::click::click(state, press),
        VIEW_METER => screen::meter::click::click(state, press),
        VIEW_PUBLISHER => screen::publisher::click::click(state, press),
        VIEW_PURCHASE => screen::purchase::click(state, press),
        VIEW_SWAP => screen::swap::click::click(state, press),
        VIEW_SEND => screen::pay::click::click(state, press),
        VIEW_NOX => screen::stake::click::click(state, press),
        VIEW_RECEIVE => receive(state, press),
        _ => home(state, press),
    }
}
