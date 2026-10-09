/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Whether the panel remembers the networks it joins. Remembering one is the
//! join's last step, on the worker that holds its passphrase
//! (`wifi_worker.rs`).

use nonos_wifi_client::sealing_ready;

use super::state::State;

/// Turn remembering on or off. It stays off on a boot that keeps nothing,
/// and turning it on first checks there is a store and a TPM key, so the
/// panel says now, not after a join, why it cannot.
pub fn toggle_remember(state: &mut State) {
    if state.wifi.remember || !state.wifi.keeps {
        state.wifi.remember = false;
        return;
    }
    match sealing_ready() {
        Ok(()) => state.wifi.remember = true,
        Err(e) => state.wifi.notice = Some(e.text()),
    }
}
