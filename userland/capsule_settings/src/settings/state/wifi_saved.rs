/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Remembering, listing and forgetting saved networks from the panel.

use nonos_wifi_client::{forget, forget_all, load, SavedError, ScanNetwork};

use super::state::State;

/// Read the saved networks' names. Opening the record asks the TPM for the
/// key, so this runs on entering the tab and after a change, not every frame.
pub fn refresh_saved(state: &mut State) {
    let w = &mut state.wifi;
    w.saved_count = 0;
    match load() {
        Ok(list) => {
            w.saved_err = None;
            for i in 0..list.len().min(w.saved.len()) {
                w.saved[i] = ScanNetwork::new(list.ssid(i), 0, !list.passphrase(i).is_empty());
                w.saved_count += 1;
            }
        }
        Err(e) => w.saved_err = Some(e),
    }
}

/// Forget the highlighted saved network, or the highlighted scanned one if it
/// is saved. A record this machine can no longer open is withdrawn whole.
pub fn forget_selected(state: &mut State) {
    let w = &state.wifi;
    let unreadable = matches!(
        w.saved_err,
        Some(SavedError::Unreadable | SavedError::Damaged | SavedError::BootChanged)
    );
    let result = if unreadable {
        Some(forget_all())
    } else {
        selected_name(state).map(|n| forget(n.ssid()))
    };
    let Some(result) = result else { return };
    state.wifi.notice = Some(match result {
        Ok(()) => "Forgotten",
        Err(e) => e.text(),
    });
    refresh_saved(state);
}

fn selected_name(state: &State) -> Option<ScanNetwork> {
    let (c, n) = (state.wifi_cursor, state.wifi_network_count);
    if c >= n {
        return state.wifi.saved.get(c - n).copied().filter(|_| c - n < state.wifi.saved_count);
    }
    let net = state.wifi_networks[c];
    state.wifi.is_saved(net.ssid()).then_some(net)
}
