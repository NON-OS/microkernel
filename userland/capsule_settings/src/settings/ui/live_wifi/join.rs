// NONOS Operating System (AGPL-3.0-or-later)

//! The join row: the pending work, the last notice, or the join's outcome.

use nonos_wifi_client::join_text;

use super::super::valbuf::ValBuf;
use crate::settings::schema::rows::Tone;
use crate::settings::state::wifi_pending::Work;
use crate::settings::state::{State, WifiConnect};

const PASS_STARS: usize = 32;

/// The passphrase being typed, masked; else what the last action did.
pub fn join(b: &mut ValBuf, state: &State) -> Tone {
    if state.wifi_pass_active {
        // Typed stars stay inside the 72-byte line with the hint after them.
        b.push_str("Passphrase: ");
        let typed = &state.wifi_pass.as_slice()[..state.wifi_pass.len.min(PASS_STARS)];
        if state.wifi_pass_shown {
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
