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

//! Opening a locked window again, once the keyring names this account.

use nonos_app_skeleton::EventOutcome;

use crate::wallet::ipc::wallet_address;
use crate::wallet::state::State;

const NO_KEYRING: &str = "The keyring did not answer for this account, so the wallet stays \
     locked. Try again in a moment.";

pub fn unlock(state: &mut State) -> EventOutcome {
    if !state.address_ready {
        state.locked = false;
        return EventOutcome::Repaint;
    }
    match wallet_address(state.vault.keyring_port, state.vault.owner_pid, state.vault.wallet_id) {
        Ok(addr) if addr == state.address => {
            state.locked = false;
            state.failure = None;
            state.net.probe_step = 1;
            crate::wallet::shield::open::ensure(state);
        }
        _ => state.failure = Some(NO_KEYRING),
    }
    EventOutcome::Repaint
}
