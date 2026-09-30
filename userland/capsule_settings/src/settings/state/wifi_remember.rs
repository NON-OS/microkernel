/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Whether the panel remembers the networks it joins, and remembering one.

use nonos_wifi_client::{remember, sealing_ready};

use super::state::State;
use super::wifi_saved::refresh_saved;

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

/// Keep a network just joined, when the person asked for that and this boot
/// keeps state. The caller wipes the passphrase after.
pub(super) fn keep_joined(state: &mut State, ssid: &[u8], pass: &[u8]) {
    if !state.wifi.remember {
        return;
    }
    state.wifi.notice = Some(match remember(ssid, pass) {
        Ok(()) => "Remembered, sealed with this machine's TPM key",
        Err(e) => e.text(),
    });
    refresh_saved(state);
}
