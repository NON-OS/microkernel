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

use crate::wifi::{scan_networks, DriverStage, ScanOutcome};

use super::state::{State, WifiConnect, WifiScan};
use super::wifi_enter::refresh_wifi_status;
use super::wifi_join::radio_on;

/// Ask the driver how far its radio came up and, only if it answered ready, scan.
/// The quick status probe is also a liveness gate: a scan is a long blocking call,
/// so it is only issued to a driver that just answered ready. A driver that
/// reported a hard failure shows its stage; one that did not answer the quick
/// probe at all is reported as unreachable rather than risking a long block on a
/// wedged driver. Every path leaves a visible result and keeps the cursor in
/// range.
pub fn run_wifi_scan(state: &mut State) {
    refresh_wifi_status(state);
    if !radio_on(state) {
        // The Wi-Fi switch is off: no scan, and no stale list to join from.
        state.wifi_network_count = 0;
        state.wifi_cursor = 0;
        state.wifi_scan = WifiScan::Idle;
        return;
    }
    // Once connected, a channel scan would retune the radio off the live link and
    // drop the connection (and net_core's traffic), so refreshing the status is
    // all a connected panel does; the scan is only for finding networks to join.
    if state.wifi_connect == WifiConnect::Connected {
        return;
    }
    match state.wifi_stage {
        Some(DriverStage::Ready) => {
            let (count, outcome, stats) = scan_networks(&mut state.wifi_networks);
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
        Some(_) => {
            // A hard bring-up failure; the stage line explains why there are none.
            state.wifi_network_count = 0;
            state.wifi_cursor = 0;
            state.wifi_scan = WifiScan::Idle;
        }
        None => {
            // The driver did not answer even the quick probe; say so rather than
            // issuing the long scan and freezing on an unresponsive driver.
            state.wifi_network_count = 0;
            state.wifi_cursor = 0;
            state.wifi_scan = WifiScan::Done(ScanOutcome::NoResponse);
        }
    }
}
