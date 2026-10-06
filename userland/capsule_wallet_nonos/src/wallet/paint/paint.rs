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

//! One screen at a time, each drawn whole on the Etna frame.

use nonos_app_skeleton::PaintBuffer;

use crate::wallet::screen;
use crate::wallet::state::{
    State, VIEW_EXPORT, VIEW_IMPORT, VIEW_NOX, VIEW_RECEIVE, VIEW_RECOVER, VIEW_SEND,
    VIEW_ACCOUNTS, VIEW_SETTINGS, VIEW_SHIELD, VIEW_SWAP,
};

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    /* What the wallet said last, drawn by the frame above the status line. */
    crate::wallet::etna::notice::set(core::str::from_utf8(state.status).unwrap_or(""));
    if state.locked {
        return screen::locked::locked(state, fb);
    }
    if state.backup_active {
        return screen::backup::backup(state, fb);
    }
    match (state.view, state.address_ready) {
        (VIEW_IMPORT | VIEW_RECOVER | VIEW_EXPORT, _) => screen::custody::show(state, fb),
        (VIEW_SETTINGS, _) => screen::settings::show(state, fb),
        (VIEW_ACCOUNTS, true) => screen::accounts::show(state, fb),
        (VIEW_RECEIVE, true) => screen::receive::receive(state, fb),
        (VIEW_SEND, true) => screen::pay::show(state, fb),
        (VIEW_SHIELD, true) => screen::shield::show::show(state, fb),
        (VIEW_SWAP, true) => screen::swap::show::show(state, fb),
        (VIEW_NOX, true) => screen::stake::show(state, fb),
        (_, true) => screen::home::home(state, fb),
        (_, false) => screen::welcome::welcome(state, fb),
    }
}
