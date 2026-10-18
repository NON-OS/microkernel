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

//! Taking up the wallet the vault opened: its address, or why there is none.

use crate::wallet::ipc::wallet_address;
use crate::wallet::state::State;

pub(super) fn adopt(state: &mut State, id: u32) {
    state.vault.restore_tried = true;
    match wallet_address(state.vault.keyring_port, state.vault.owner_pid, id) {
        Ok(addr) => {
            state.vault.wallet_id = id;
            state.acct.address = addr;
            state.acct.address_ready = true;
            state.vault.saved = true;
            state.status = b"wallet restored from this machine";
            crate::wallet::accounts::restore(state);
        }
        /*
         * The keyring took the key but will not name it, which is a keyring
         * fault rather than a vault one; leave the wallet unset rather than
         * show an id with no address behind it.
         */
        Err(_) => {
            state.vault.present = true;
            state.status = b"vault opened but the address could not be read";
        }
    }
}
