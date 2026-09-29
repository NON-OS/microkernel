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

use crate::wifi::connect_network;

use super::cache::FieldValue;
use super::cached_value::cached_value;
use super::edit_buffer::EditBuffer;
use super::state::{State, WifiConnect};

/// Begin or complete a connection to the selected network. A secured network
/// first opens the passphrase editor; the second call (or an open network on the
/// first) sends the driver the SSID and passphrase and runs the whole join, which
/// blocks for a few seconds. The result is recorded for the panel.
pub fn connect_selected(state: &mut State) {
    if state.wifi_network_count == 0 || !radio_on(state) {
        return;
    }
    let idx = state.wifi_cursor.min(state.wifi_network_count - 1);
    let secured = state.wifi_networks[idx].secured;
    // A secured network needs a passphrase: open the editor on the first Enter.
    if secured && !state.wifi_pass_active {
        state.wifi_pass_active = true;
        state.wifi_pass = EditBuffer::empty();
        return;
    }
    // The passphrase is in (or the network is open): join now. The driver call
    // blocks for the length of the handshake, and the key handler returns Repaint
    // straight after, so the outcome is painted the same frame the join finishes.
    let idx = state.wifi_cursor.min(state.wifi_network_count - 1);
    let mut ssid = [0u8; 32];
    let slen = {
        let s = state.wifi_networks[idx].ssid();
        let n = s.len().min(32);
        ssid[..n].copy_from_slice(&s[..n]);
        n
    };
    let result = connect_network(&ssid[..slen], state.wifi_pass.as_slice());
    state.wifi_connect =
        if result.code == 0 { WifiConnect::Connected } else { WifiConnect::Failed(result) };
    state.wifi_pass_active = false;
}

// Off only when the store says so; an unread value does not block the radio.
pub(super) fn radio_on(state: &State) -> bool {
    !matches!(cached_value(state, Field::WifiRadio), FieldValue::Bool(false))
}
