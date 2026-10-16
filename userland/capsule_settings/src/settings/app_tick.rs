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

//! Settings' pace: how soon the skeleton calls again, quicker while a Wi-Fi
//! scan or join is asked for or out with the worker, and what a tick does.

use super::app_ready::Settings;
use super::section::Section;
use super::state::wifi::status::refresh_wifi_status;
use super::state::wifi::step::step_wifi;

/// Between the ticks that show and then run a pending Wi-Fi scan or join.
const WIFI_TICK_MS: i64 = 30;
/// Between looks for the answer of a scan or join out on the worker. Not
/// sooner: the worker shares the processor, and its answer takes seconds.
const WIFI_OUT_TICK_MS: i64 = 100;

impl Settings {
    // While the Wi-Fi or Network page is open, re-poll net_core every tick so a
    // lease that binds a few seconds after connecting shows its address without
    // the user having to trigger another scan.
    // A scan or join asked for is shown and run on the next two ticks, so
    // those come quickly rather than a second apart.
    // One handed to the worker is looked for every 100 ms until it answers.
    pub(super) fn tick_interval(&self) -> i64 {
        let pending = self.state.wifi.pending;
        if pending.is_asked() {
            WIFI_TICK_MS
        } else if pending.is_out() {
            WIFI_OUT_TICK_MS
        } else {
            1000
        }
    }

    // Only a request still to be shown and handed over wants the runner to
    // yield rather than sleep: one that is out is the worker's to advance.
    pub(super) fn wifi_busy(&self) -> bool {
        self.state.wifi.pending.is_asked()
    }

    pub(super) fn tick(&mut self) -> bool {
        // A pass the policy store did not answer runs again from here, so the
        // stored values replace the defaults without a key or a click.
        if self.state.policy_ready && !self.hydrated && self.hydrate_due() {
            self.ensure_ready();
            return true;
        }
        // A scan or join asked for runs from here, a tick after the panel
        // said it would, whatever page is open by then.
        if step_wifi(&mut self.state) {
            return true;
        }
        if matches!(self.state.section, Section::Wifi | Section::Network) {
            refresh_wifi_status(&mut self.state);
            return true;
        }
        false
    }
}
