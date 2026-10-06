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

use crate::wifi::{DriverStage, ScanOutcome};

use nonos_libc::mk_time_millis;

use super::state::{State, WifiScan};
use super::wifi_answer::{hand_off, refuse_while_out};
use super::wifi_enter::refresh_wifi_status;
use super::wifi_join::radio_on;
use super::wifi_pending::{Pending, Work};
use super::wifi_refusal::scan_refusal;
use super::wifi_worker::Ask;

/// Enter on the Wi-Fi panel: settle at once what needs no radio work, and
/// leave a scan to the ticks (`wifi_pending.rs`), so "Scanning..." is on the
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

/// Ask the driver how far its radio came up and, only if it answered ready, hand
/// the scan to the worker (`wifi_worker.rs`). The quick status probe is also a
/// liveness gate: a scan is a long call, so it is only issued to a driver that
/// just answered ready. A driver that reported a hard failure shows its stage;
/// one that did not answer the quick probe at all is reported as unreachable
/// rather than tying the worker up on a wedged driver. Every path leaves a
/// visible result and keeps the cursor in range.
pub fn scan_now(state: &mut State) {
    refresh_wifi_status(state);
    if !radio_on(state) || state.wifi.joined().is_some() {
        return;
    }
    match (state.wifi_stage, state.wifi.driver) {
        (Some(DriverStage::Ready), Some(driver)) => hand_off(state, Work::Scan, Ask::Scan(driver)),
        (Some(DriverStage::Ready), None) => {
            state.wifi_network_count = 0;
            state.wifi_cursor = 0;
            state.wifi_scan = WifiScan::Done(ScanOutcome::NoService);
        }
        (Some(_), _) => {
            // A hard bring-up failure; the stage line explains why there are none.
            state.wifi_network_count = 0;
            state.wifi_cursor = 0;
            state.wifi_scan = WifiScan::Idle;
        }
        (None, _) => {
            // The driver did not answer even the quick probe; say so rather than
            // issuing the long scan to an unresponsive driver.
            state.wifi_network_count = 0;
            state.wifi_cursor = 0;
            state.wifi_scan = WifiScan::Done(ScanOutcome::NoResponse);
        }
    }
}
