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

use nonos_wifi_client::{find, keeps_state};

use crate::wifi::{driver_datapath, net_status, scan_adapters};

use super::state::State;
use super::wifi_saved::refresh_saved;

/// Re-enumerate the wireless adapters into the WiFi panel state and keep the
/// selection cursor inside the new list.
pub fn refresh_wifi(state: &mut State) {
    state.wifi_adapter_count = scan_adapters(&mut state.wifi_adapters);
    if state.wifi_cursor >= state.wifi_adapter_count {
        state.wifi_cursor = state.wifi_adapter_count.saturating_sub(1);
    }
}

/// Switch to the Wi-Fi tab and enumerate adapters. Does not scan here: a scan is a
/// blocking request to the driver, and running it on tab entry would freeze the
/// whole app if the driver were slow to answer. The user starts a scan with Enter,
/// which keeps the app responsive while navigating. Leaves editing behind, like
/// selecting any other section.
pub fn enter_wifi(state: &mut State) {
    state.editing = false;
    refresh_wifi(state);
    refresh_wifi_status(state);
    refresh_saved(state);
}

/// Refresh which driver runs, its bring-up stage and link, whether this boot
/// keeps state, the data-path frame counts and net_core's lease without
/// touching the radio, so the connected view (address, counters)
/// stays current on a live link that a channel scan would otherwise drop.
pub fn refresh_wifi_status(state: &mut State) {
    let driver = find();
    state.wifi.driver = driver;
    state.wifi_stage = driver.and_then(|d| d.stage());
    state.wifi.link = driver.and_then(|d| d.link());
    state.wifi.keeps = keeps_state();
    state.wifi.remember &= state.wifi.keeps;
    state.wifi_datapath = driver_datapath();
    state.wifi_net = net_status();
}
