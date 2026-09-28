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

//! A click or a scroll on a screen drawn on the Etna frame. The old chrome's
//! header icons and side rail are not on these screens, so their hit zones
//! must not answer here: this handler runs instead of them, not before them.

use nonos_app_skeleton::clients::clipboard::clipboard_copy;
use nonos_app_skeleton::EventOutcome;

use crate::wallet::screen::hits::{at, Press};
use crate::wallet::state::{State, VIEW_HOME, VIEW_RECEIVE, VIEW_SEND, VIEW_SHIELD, VIEW_SWAP};

fn go(state: &mut State, view: u8) -> EventOutcome {
    state.view = view;
    state.scroll = 0;
    EventOutcome::Repaint
}

pub fn etna_click(state: &mut State, x: u32, y: u32) -> EventOutcome {
    let Some(press) = at(x, y) else {
        return EventOutcome::Idle;
    };
    if state.backup_active {
        if press != Press::Footer(0) {
            return EventOutcome::Idle;
        }
        let _ = super::backup::confirm_backup(state);
        return go(state, VIEW_HOME);
    }
    if state.view == VIEW_SHIELD {
        return crate::wallet::screen::shield::click::click(state, press);
    }
    match (press, state.view) {
        (Press::Footer(0), VIEW_HOME) if !state.address_ready => super::generate::generate(state),
        (Press::Footer(1), VIEW_HOME) if !state.address_ready => {
            super::import::toggle_import(state)
        }
        (Press::Footer(0), VIEW_RECEIVE) => {
            let hex = crate::wallet::screen::receive_address::address_hex(state);
            let _ = clipboard_copy(hex.as_bytes());
            state.status = b"address copied";
            EventOutcome::Repaint
        }
        (Press::Back, _) => go(state, VIEW_HOME),
        (Press::Send, _) => go(state, VIEW_SEND),
        (Press::Receive, _) => go(state, VIEW_RECEIVE),
        (Press::Swap, _) => go(state, VIEW_SWAP),
        (Press::Shield, _) => crate::wallet::screen::shield::click::open(state),
        (Press::Settings | Press::Accounts, _) => {
            state.panel = 3;
            EventOutcome::Repaint
        }
        _ => EventOutcome::Idle,
    }
}
