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

//! The typed private key, handed to the keyring: decoded into a secret that
//! is zeroed on every path, and the typed copy wiped with it.

use nonos_app_skeleton::EventOutcome;
use zeroize::Zeroize;

use super::import::wipe;
use crate::wallet::ipc::{forget_key, import_wallet, wallet_address};
use crate::wallet::state::State;

pub(in crate::wallet::event) fn submit(state: &mut State) -> EventOutcome {
    if state.custody.import_len != 64 {
        state.status = b"private key must be 64 hex chars";
        return EventOutcome::Repaint;
    }
    let mut secret = [0u8; 32];
    for i in 0..32 {
        let hi = super::super::hex_digit::hex_digit(state.custody.import_hex[i * 2] as u32);
        let lo = super::super::hex_digit::hex_digit(state.custody.import_hex[i * 2 + 1] as u32);
        match (hi, lo) {
            (Some(h), Some(l)) => secret[i] = (h << 4) | l,
            _ => {
                secret.zeroize();
                wipe(state);
                state.status = b"invalid private key";
                return EventOutcome::Repaint;
            }
        }
    }
    let outcome = match import_wallet(state.vault.keyring_port, state.vault.owner_pid, &secret) {
        Ok(id) => match wallet_address(state.vault.keyring_port, state.vault.owner_pid, id) {
            Ok(addr) => {
                state.vault.wallet_id = id;
                state.acct.address = addr;
                state.acct.address_ready = true;
                state.import_active = false;
                state.status = b"wallet imported";
                super::super::locking::keep::keep(state, true);
                super::super::probe::probe_tick::probe_kick(state)
            }
            Err(_) => {
                let gone = forget_key(state.vault.keyring_port, state.vault.owner_pid, id).is_ok();
                state.status = super::keyring_says::unnamed(gone);
                EventOutcome::Repaint
            }
        },
        Err(code) => {
            state.status = super::keyring_says::refusal(
                code,
                b"the keyring rejected this key: it is not a valid secp256k1 key",
            );
            EventOutcome::Repaint
        }
    };
    secret.zeroize();
    wipe(state);
    outcome
}
