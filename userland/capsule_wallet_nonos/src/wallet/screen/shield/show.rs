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

/* Paint whichever Shield screen is up. */

use nonos_app_skeleton::PaintBuffer;

use crate::wallet::state::shield_ui::*;
use crate::wallet::state::State;

pub fn show(state: &State, fb: &mut PaintBuffer) {
    match state.shield_ui.screen {
        SHIELD_DEPOSIT => super::deposit::deposit(state, fb),
        SHIELD_SEND => super::send::send(state, fb),
        SHIELD_WITHDRAW => super::withdraw::withdraw(state, fb),
        SHIELD_REVIEW => super::review::review(state, fb),
        SHIELD_PROVING => super::proving::proving(state, fb),
        SHIELD_HISTORY => super::history::history(state, fb),
        SHIELD_NETWORK => super::network::network(state, fb),
        _ => super::home::home(state, fb),
    }
}
