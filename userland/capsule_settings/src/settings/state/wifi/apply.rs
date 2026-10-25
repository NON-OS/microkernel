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

//! The driver's answer put on the panel: a scan's networks, strongest first,
//! with the cursor kept on the list; or a join's result, and the saved list
//! and status refreshed after it.

use super::super::state::{State, WifiConnect, WifiScan};
use super::saved::refresh_saved;
use super::status::refresh_wifi_status;
use super::worker_msg::Answer;

/// Put the driver's answer on the panel.
pub(super) fn apply(state: &mut State, answer: Answer) {
    match answer {
        Answer::Scanned { nets, count, outcome, stats } => {
            let count = count.min(state.wifi_networks.len());
            state.wifi_networks[..count].copy_from_slice(&nets[..count]);
            // Strongest signal first, so the most reachable networks head the list
            // once the driver reports real RSSI.
            state.wifi_networks[..count].sort_unstable_by(|a, b| b.signal.cmp(&a.signal));
            state.wifi_network_count = count;
            state.wifi_scan = WifiScan::Done(outcome);
            state.wifi_diag.stats = stats;
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
