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

use super::generate_draw::draw;
use crate::wallet::ipc::{forget_key, wallet_address};
use crate::wallet::state::State;

pub fn generate(state: &mut State) -> EventOutcome {
    let (id, count) = match draw(state) {
        Ok(drawn) => drawn,
        Err(last_err) => {
            super::backup::wipe_backup(state);
            /* The keyring's EINVAL here is a draw it could not make. */
            state.status = super::keyring_says::refusal(
                last_err,
                b"the keyring could not draw entropy for a new wallet",
            );
            return EventOutcome::Repaint;
        }
    };
    match wallet_address(state.vault.keyring_port, state.vault.owner_pid, id) {
        Ok(addr) => {
            state.vault.wallet_id = id;
            state.address = addr;
            state.address_ready = true;
            state.view = crate::wallet::state::VIEW_RECEIVE;
            /*
             * Show the one-time backup screen before anything else; the words
             * are wiped the moment the user confirms they are written down.
             */
            state.custody.backup_count = count;
            state.backup_active = true;
            state.status = b"wallet created, write down the phrase";
            super::super::locking::keep::keep(state, false);
            super::super::probe::probe_tick::probe_kick(state)
        }
        Err(_) => {
            super::backup::wipe_backup(state);
            let gone = forget_key(state.vault.keyring_port, state.vault.owner_pid, id).is_ok();
            state.status = super::keyring_says::unnamed(gone);
            EventOutcome::Repaint
        }
    }
}
