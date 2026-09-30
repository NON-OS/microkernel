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

use nonos_policy_proto::Field;
use nonos_wifi_client::wipe;

use super::cache::FieldValue;
use super::cached_value::cached_value;
use super::state::{State, WifiConnect};
use super::wifi_enter::refresh_wifi_status;
use super::wifi_remember::keep_joined;

/// Begin or complete a join of the highlighted network. A secured network
/// first opens the passphrase editor; the second call (or an open network on
/// the first) runs the whole join, which blocks for a few seconds. The
/// passphrase is wiped once the join (and any remembering) is done.
pub fn connect_selected(state: &mut State) {
    let Some(driver) = state.wifi.driver else { return };
    if state.wifi_cursor >= state.wifi_network_count || !radio_on(state) {
        return;
    }
    let net = state.wifi_networks[state.wifi_cursor];
    if net.secured && !state.wifi_pass_active {
        clear_passphrase(state);
        state.wifi_pass_active = true;
        return;
    }
    state.wifi.notice = None;
    let mut pass = state.wifi_pass;
    clear_passphrase(state);
    let result = driver.connect(net.ssid(), pass.as_slice());
    state.wifi_connect =
        if result.code == 0 { WifiConnect::Connected } else { WifiConnect::Failed(result) };
    if result.code == 0 {
        keep_joined(state, net.ssid(), pass.as_slice());
    }
    wipe(&mut pass.bytes);
    refresh_wifi_status(state);
}

/// Close the passphrase editor and wipe what was typed.
pub fn clear_passphrase(state: &mut State) {
    wipe(&mut state.wifi_pass.bytes);
    state.wifi_pass.len = 0;
    state.wifi_pass_active = false;
}

/// Leave the network the radio is associated with and clear its keys.
pub fn leave(state: &mut State) {
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
