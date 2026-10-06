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
//! A press on Settings.

use nonos_app_skeleton::EventOutcome;

use crate::wallet::screen::hits::Press;
use crate::wallet::state::{
    switch_network, State, VIEW_EXPORT, VIEW_HOME, VIEW_IMPORT, VIEW_RECOVER,
};

pub fn click(state: &mut State, press: Press) -> EventOutcome {
    match press {
        Press::Back => state.view = VIEW_HOME,
        Press::Dismiss => state.failure = None,
        Press::Network(n) => match switch_network(state, n == 1) {
            Ok(true) => state.status = b"network changed",
            Ok(false) => {}
            Err(why) => state.failure = Some(why),
        },
        /* The key screen asks first; only its own button shows the key. */
        Press::Row(0) => return crate::wallet::screen::custody::click::open(state, VIEW_EXPORT),
        Press::Row(1) => return crate::wallet::screen::custody::click::open(state, VIEW_RECOVER),
        Press::Row(2) => return crate::wallet::screen::custody::click::open(state, VIEW_IMPORT),
        Press::Row(3) => return crate::wallet::event::lock(state),
        _ => return EventOutcome::Idle,
    }
    state.scroll = 0;
    EventOutcome::Repaint
}
