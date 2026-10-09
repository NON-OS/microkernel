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

//! Locking the window, and opening it again.
//!
//! A lock puts away everything secret the window holds: a key shown or
//! being typed, recovery words being typed, a review not yet signed, and
//! the shield's open store. Opening it again asks the keyring to name this
//! account, so a window whose keyring is gone stays locked. The keyring has
//! no passphrase of its own yet, so this keeps a passer-by from reading the
//! screen; it is not a lock on the keys themselves.

use nonos_app_skeleton::EventOutcome;

use crate::wallet::ipc::wallet_address;
use crate::wallet::state::{State, VIEW_HOME};

const NO_KEYRING: &str = "The keyring did not answer for this account, so the wallet stays \
     locked. Try again in a moment.";

pub fn lock(state: &mut State) -> EventOutcome {
    if state.export_active {
        let _ = super::toggle_export(state);
    }
    if state.import_active {
        let _ = super::toggle_import(state);
    }
    if state.recover_active {
        let _ = super::toggle_recover(state);
    }
    /* A review is signed only by the holder who read it. A payment already
     * going out goes on, and is said when the window opens. */
    if !crate::wallet::act::running(state) {
        state.send_draft = None;
        state.send_stage = crate::wallet::send::STAGE_FORM;
        state.stake_draft = None;
        state.tx_raw.clear();
        state.tx_ready = false;
    }
    crate::wallet::shield::open::lock(state);
    state.custody_armed = false;
    state.locked = true;
    state.view = VIEW_HOME;
    state.scroll = 0;
    state.failure = None;
    EventOutcome::Repaint
}

pub fn unlock(state: &mut State) -> EventOutcome {
    if !state.address_ready {
        state.locked = false;
        return EventOutcome::Repaint;
    }
    match wallet_address(state.keyring_port, state.owner_pid, state.wallet_id) {
        Ok(addr) if addr == state.address => {
            state.locked = false;
            state.failure = None;
            state.probe_step = 1;
            crate::wallet::shield::open::ensure(state);
        }
        _ => state.failure = Some(NO_KEYRING),
    }
    EventOutcome::Repaint
}
