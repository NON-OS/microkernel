/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The Wi-Fi page's live rows: the join in progress or its outcome, and
//! whether joins are remembered (the link's row is `live_link.rs`).

use nonos_wifi_client::join_text;

use crate::settings::schema::rows::Tone;
use crate::settings::state::wifi::pending::Work;
use crate::settings::state::{State, WifiConnect};

use super::valbuf::ValBuf;

#[path = "live_notice.rs"]
mod live_notice;

const PASS_STARS: usize = 32;

/// The passphrase being typed, masked; else what the last action did.
pub fn join(b: &mut ValBuf, state: &State) -> Tone {
    if state.wifi_pass.active {
        // Typed stars stay inside the 72-byte line with the hint after them.
        b.push_str("Passphrase: ");
        let typed = &state.wifi_pass.text.as_slice()[..state.wifi_pass.text.len.min(PASS_STARS)];
        if state.wifi_pass.shown {
            b.push_bytes(typed);
        } else {
            for _ in typed {
                b.push_str("*");
            }
        }
        b.push_str("_  Tab shows, Enter joins");
        return Tone::Warn;
    }
    match state.wifi.pending.work() {
        Some(Work::Scan) => {
            b.push_str("Scanning...");
            return Tone::Idle;
        }
        Some(Work::Join) => {
            b.push_str("Joining ");
            b.push_bytes(state.wifi.join_net.ssid());
            b.push_str("...");
            return Tone::Idle;
        }
        None => {}
    }
    if let Some(notice) = state.wifi.notice {
        live_notice::say(b, state, notice);
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
