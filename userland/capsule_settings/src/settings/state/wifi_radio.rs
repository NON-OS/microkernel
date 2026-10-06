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

//! The Wi-Fi switch turned off. It only stopped new scans and joins before,
//! so a joined network stayed up while the card's badge said Off. Off now
//! leaves that network too, and drops the list a join would be made from.

use super::state::{State, WifiScan};
use super::wifi_join::leave;

pub fn radio_switched_off(state: &mut State) {
    if state.wifi.joined().is_some() {
        leave(state);
    }
    state.wifi_network_count = 0;
    state.wifi_cursor = 0;
    state.wifi_scan = WifiScan::Idle;
}
