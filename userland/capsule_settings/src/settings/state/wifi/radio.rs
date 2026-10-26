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

//! The radio: whether the store has it on, the network it is on left, and
//! the switch turned off. Off only stopped new scans and joins before, so a
//! joined network stayed up while the card's badge said Off. Off now leaves
//! that network too, and drops the list a join would be made from.

use super::super::cache::FieldValue;
use super::super::cached_value::cached_value;
use super::super::model::{State, WifiConnect, WifiScan};
use super::answer::refuse_while_out;
use super::status::refresh_wifi_status;
use nonos_policy_proto::Field;

pub fn radio_switched_off(state: &mut State) {
    if state.wifi.joined().is_some() {
        leave(state);
    }
    state.wifi_network_count = 0;
    state.wifi_cursor = 0;
    state.wifi_scan = WifiScan::Idle;
}

/// Leave the network the radio is associated with and clear its keys.
pub fn leave(state: &mut State) {
    if refuse_while_out(state) {
        return;
    }
    if let Some(driver) = state.wifi.driver {
        driver.disconnect();
    }
    state.wifi_connect = WifiConnect::Idle;
    refresh_wifi_status(state);
}

/*
 * Off only when the store says so; an unread value does not block the radio.
 */
pub(super) fn radio_on(state: &State) -> bool {
    !matches!(cached_value(state, Field::WifiRadio), FieldValue::Bool(false))
}
