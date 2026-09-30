/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The Wi-Fi page's live rows: the link, the join in progress or its outcome,
//! and whether joins are remembered.

use nonos_wifi_client::{join_text, DriverStage};

use crate::settings::schema::rows::Tone;
use crate::settings::state::{State, WifiConnect};

use super::valbuf::ValBuf;

/// Which driver runs and where its link stands.
pub fn link(b: &mut ValBuf, state: &State) -> Tone {
    let Some(driver) = state.wifi.driver else {
        b.push_str("No Wi-Fi driver running");
        return Tone::Idle;
    };
    if let Some(ssid) = state.wifi.joined() {
        b.push_str("Connected to ");
        b.push_bytes(ssid);
        return Tone::Ok;
    }
    if state.wifi_stage == Some(DriverStage::Ready) {
        b.push_str("Not connected");
        return Tone::Idle;
    }
    b.push_str(driver.label());
    b.push_str(": ");
    b.push_str(state.wifi_stage.map_or("not answering", |s| s.text()));
    Tone::Warn
}

/// The passphrase being typed, masked; else what the last action did.
pub fn join(b: &mut ValBuf, state: &State) -> Tone {
    if state.wifi_pass_active {
        b.push_str("Passphrase: ");
        for _ in 0..state.wifi_pass.len {
            b.push_str("*");
        }
        return Tone::Idle;
    }
    if let Some(notice) = state.wifi.notice {
        b.push_str(notice);
        return Tone::Idle;
    }
    match state.wifi_connect {
        WifiConnect::Idle => b.push_str("--"),
        WifiConnect::Connected => b.push_str(join_text(0)),
        WifiConnect::Failed(r) => {
            b.push_str(join_text(r.code));
            return Tone::Warn;
        }
    }
    Tone::Idle
}

/// Remembering is offered only on a boot that keeps state.
pub fn remember(b: &mut ValBuf, state: &State) -> Tone {
    match (state.wifi.keeps, state.wifi.remember) {
        (false, _) => b.push_str("Off: this boot keeps nothing"),
        (true, true) => b.push_str("On"),
        (true, false) => b.push_str("Off"),
    }
    Tone::Idle
}
