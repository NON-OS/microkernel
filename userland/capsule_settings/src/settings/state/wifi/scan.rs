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

use super::super::state::{State, WifiScan};
use super::answer::refuse_while_out;
use super::pending::{Pending, Work};
use super::radio::radio_on;
use super::refusal::scan_refusal;
use super::status::refresh_wifi_status;
use nonos_libc::mk_time_millis;

/// Enter on the Wi-Fi panel: settle at once what needs no radio work, and
/// leave a scan to the ticks (`state/wifi/pending.rs`), so "Scanning..." is on the
/// window before the driver is asked.
pub fn run_wifi_scan(state: &mut State) {
    if refuse_while_out(state) {
        return;
    }
    if let Some(why) = scan_refusal(radio_on(state)) {
        // The Wi-Fi switch is off: no scan, and no stale list to join from.
        state.wifi_network_count = 0;
        state.wifi_cursor = 0;
        state.wifi_scan = WifiScan::Idle;
        state.wifi.notice = Some(why);
        return;
    }
    // Once connected, a channel scan would retune the radio off the live link and
    // drop the connection (and net_core's traffic), so refreshing the status is
    // all a connected panel does; the scan is only for finding networks to join.
    if state.wifi.joined().is_some() {
        refresh_wifi_status(state);
        return;
    }
    state.wifi.pending = Pending::ask(Work::Scan, mk_time_millis());
}
