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

//! The phrase's word indices handed to the keyring, and the wallet it
//! derives taken up, or the keyring's refusal said.

use nonos_app_skeleton::EventOutcome;

use crate::wallet::ipc::{forget_key, recover_wallet, wallet_address};
use crate::wallet::state::State;

/// Recover from `indices` (12 to 24 of them, already checked).
pub(super) fn recovered(state: &mut State, indices: &[u16]) -> EventOutcome {
    match recover_wallet(state.keyring_port, state.owner_pid, indices) {
        Ok(id) => match wallet_address(state.keyring_port, state.owner_pid, id) {
            Ok(addr) => {
                state.wallet_id = id;
                state.address = addr;
                state.address_ready = true;
                state.recover_active = false;
                state.status = b"wallet recovered";
                super::keep::keep(state, false);
                super::probe_tick::probe_kick(state)
            }
            Err(_) => {
                let gone = forget_key(state.keyring_port, state.owner_pid, id).is_ok();
                state.status = super::keyring_says::unnamed(gone);
                EventOutcome::Repaint
            }
        },
        Err(code) => {
            state.status = super::keyring_says::refusal(
                code,
                b"the keyring rejected this phrase: its checksum does not match",
            );
            EventOutcome::Repaint
        }
    }
}
