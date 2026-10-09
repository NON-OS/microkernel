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

use nonos_libc::mk_uptime_ms;

use crate::wifi::{driver_datapath, net_poll_due, net_status, scan_adapters};

use super::state::State;
use super::wifi_saved::refresh_saved;

/// Re-enumerate the wireless adapters into the WiFi panel state.
pub fn refresh_wifi(state: &mut State) {
    state.wifi_adapter_count = scan_adapters(&mut state.wifi_adapters);
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
    let rows = state.wifi_network_count + state.wifi.saved_count;
    state.wifi_cursor = super::wifi_cursor::kept_in(state.wifi_cursor, rows);
}

/// Refresh which driver runs, its bring-up stage and link, whether this boot
/// keeps state, the data-path frame counts and net_core's lease without
/// touching the radio, so the connected view (address, counters)
/// stays current on a live link that a channel scan would otherwise drop.
///
/// While a scan or join is out on the worker the driver is busy answering
/// it, and a probe now would sit behind it on the window thread; the last
/// driver readings stay until the answer comes. Whether this boot keeps
/// state is not asked then either: it does not change during a join, and it
/// was asked of the policy store every 100 ms.
pub fn refresh_wifi_status(state: &mut State) {
    if state.wifi.pending.is_out() {
        poll_net(state);
        return;
    }
    let driver = find();
    state.wifi.driver = driver;
    state.wifi_stage = driver.and_then(|d| d.stage());
    state.wifi.link = driver.and_then(|d| d.link());
    state.wifi.keeps = keeps_state();
    state.wifi.remember &= state.wifi.keeps;
    state.wifi_datapath = driver_datapath();
    poll_net(state);
}

/* The lease, asked at most once a second (wifi/net_poll.rs): this runs on
 * every tick of the Wi-Fi and Network pages, 100 ms apart while a join is
 * out, and the DHCP client is busy getting the address just then. */
fn poll_net(state: &mut State) {
    let now = mk_uptime_ms();
    if !net_poll_due(state.wifi_net_polled_ms, now) {
        return;
    }
    state.wifi_net_polled_ms = Some(now);
    state.wifi_net = net_status();
}
