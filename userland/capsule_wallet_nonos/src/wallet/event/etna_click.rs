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

//! A click on a screen drawn on the Etna frame. The old chrome's header
//! icons and side rail are not on these screens, so their hit zones must
//! not answer here: this handler runs instead of them, not before them.

use nonos_app_skeleton::EventOutcome;

use crate::wallet::screen::hits::{at, Press};
use crate::wallet::state::{State, VIEW_RECEIVE, VIEW_SEND, VIEW_SHIELD, VIEW_SWAP};

pub fn etna_click(state: &mut State, x: u32, y: u32) -> EventOutcome {
    let Some(press) = at(x, y) else {
        return EventOutcome::Idle;
    };
    match press {
        Press::Footer(0) if !state.address_ready => return super::generate::generate(state),
        Press::Footer(1) if !state.address_ready => return super::import::toggle_import(state),
        Press::Send => state.view = VIEW_SEND,
        Press::Receive => state.view = VIEW_RECEIVE,
        Press::Swap => state.view = VIEW_SWAP,
        Press::Shield => state.view = VIEW_SHIELD,
        Press::Settings | Press::Accounts => state.panel = 3,
        _ => return EventOutcome::Idle,
    }
    EventOutcome::Repaint
}
