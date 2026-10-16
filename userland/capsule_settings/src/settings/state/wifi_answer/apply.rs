// NONOS Operating System (AGPL-3.0-or-later)

//! Putting the driver's answer on the panel.

use super::super::state::{State, WifiConnect, WifiScan};
use super::super::wifi_enter::refresh_wifi_status;
use super::super::wifi_saved::refresh_saved;
use super::super::wifi_worker::Answer;

/// Put the driver's answer on the panel.
pub(in super::super) fn apply(state: &mut State, answer: Answer) {
    match answer {
        Answer::Scanned { nets, count, outcome, stats } => {
            let count = count.min(state.wifi_networks.len());
            state.wifi_networks[..count].copy_from_slice(&nets[..count]);
            // Strongest signal first, so the most reachable networks head the list
            // once the driver reports real RSSI.
            state.wifi_networks[..count].sort_unstable_by(|a, b| b.signal.cmp(&a.signal));
            state.wifi_network_count = count;
            state.wifi_scan = WifiScan::Done(outcome);
            state.wifi_stats = stats;
            if state.wifi_cursor >= count {
                state.wifi_cursor = count.saturating_sub(1);
            }
        }
        Answer::Joined { result, kept, dropped } => {
            state.wifi_connect =
                if result.code == 0 { WifiConnect::Connected } else { WifiConnect::Failed(result) };
            if let Some(said) = kept {
                state.wifi.notice = Some(said);
                state.wifi.dropped = dropped;
                refresh_saved(state);
            }
            refresh_wifi_status(state);
        }
    }
}
