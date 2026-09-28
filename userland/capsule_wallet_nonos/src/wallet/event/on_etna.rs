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
 * Whether the screen on show is drawn on the Etna frame. Everything else
 * still paints the older chrome, so this decides which click handler and
 * which painter a frame gets.
 */

use crate::wallet::state::{State, VIEW_HOME, VIEW_RECEIVE, VIEW_SHIELD, VIEW_SWAP};

pub fn on_etna(state: &State) -> bool {
    state.panel == 0
        && (state.backup_active
            || state.view == VIEW_HOME
            || ((state.view == VIEW_SHIELD || state.view == VIEW_SWAP) && state.address_ready)
            || (state.view == VIEW_RECEIVE && !state.import_active))
}
