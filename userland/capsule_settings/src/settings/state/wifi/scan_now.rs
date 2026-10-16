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

//! The scan a tick runs: the driver asked first whether it is ready, and the
//! long scan handed to the worker only then; every other path shows why there
//! is no list.

use super::super::state::{State, WifiScan};
use super::answer::hand_off;
use super::pending::Work;
use super::radio::radio_on;
use super::status::refresh_wifi_status;
use super::worker::Ask;
use crate::wifi::{DriverStage, ScanOutcome};

/// Ask the driver how far its radio came up and, only if it answered ready, hand
/// the scan to the worker (`state/wifi/worker.rs`). The quick status probe is also a
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
    match (state.wifi_diag.stage, state.wifi.driver) {
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
