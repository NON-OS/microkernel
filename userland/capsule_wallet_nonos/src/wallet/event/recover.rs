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

use nonos_app_skeleton::EventOutcome;
use zeroize::Zeroize;

use crate::wallet::state::{State, VIEW_RECEIVE};

// Recovery-phrase entry. Words are typed in the clear so the user can check
// them (a phrase is only ever typed to RECOVER, on a machine the user already
// trusts with the resulting key), wiped on submit or cancel. Each word is
// resolved against the BIP39 list locally for immediate feedback; the
// keyring re-validates the checksum before deriving anything.

pub fn toggle_recover(state: &mut State) -> EventOutcome {
    state.recover_active = !state.recover_active;
    if state.recover_active {
        state.view = VIEW_RECEIVE;
        state.import_active = false;
        state.status = b"type your 12-24 word phrase, Enter to recover";
    } else {
        wipe(state);
    }
    EventOutcome::Repaint
}

pub(super) fn wipe(state: &mut State) {
    state.recover_buf.zeroize();
    state.recover_len = 0;
}
