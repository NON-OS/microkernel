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
use nonos_libc::mk_time_millis;

use super::wifi_answer::{hand_off, refuse_while_out};
use super::wifi_enter::refresh_wifi_status;
use super::wifi_pending::{Pending, Work};
use super::wifi_refusal::join_refusal;
use super::wifi_worker::{Ask, Secret};

/// Begin a join of the highlighted network. A secured network first opens
/// the passphrase editor; the second call (or an open network on the first)
/// asks for the join, which a tick runs (`join_now`) once the panel has
/// said "Joining". The passphrase stays in its buffer until then.
pub fn connect_selected(state: &mut State) {
    if refuse_while_out(state) {
        return;
    }
    let on_row = state.wifi_cursor < state.wifi_network_count;
    if let Some(why) = join_refusal(radio_on(state), state.wifi.driver.is_some(), on_row) {
        state.wifi.notice = Some(why);
        return;
    }
    let net = state.wifi_networks[state.wifi_cursor];
    if net.secured && !state.wifi_pass_active {
        // A join still waiting would run with the buffer this clears.
        if state.wifi.pending.work() == Some(Work::Join) {
            state.wifi.pending = Pending::Idle;
        }
        clear_passphrase(state);
        state.wifi_pass_active = true;
        return;
    }
    state.wifi.notice = None;
    state.wifi_pass_active = false;
    state.wifi.join_net = net;
    state.wifi.pending = Pending::ask(Work::Join, mk_time_millis());
}

/// Hand the join asked for to the worker (`wifi_worker.rs`): association and
/// the four-way handshake answer only when done, a few seconds or more. The
/// passphrase moves into the request, which wipes it when dropped, and the
/// panel's buffer is wiped now; remembering, when asked for, happens on the
/// worker while it still holds the passphrase.
pub fn join_now(state: &mut State) {
    let net = state.wifi.join_net;
    let pass = Secret::new(state.wifi_pass.as_slice());
    clear_passphrase(state);
    let Some(driver) = state.wifi.driver.filter(|_| radio_on(state)) else {
        return;
    };
    let remember = state.wifi.remember;
    hand_off(state, Work::Join, Ask::Join { driver, net, pass, remember });
}

/// Close the passphrase editor and wipe what was typed.
pub fn clear_passphrase(state: &mut State) {
    wipe(&mut state.wifi_pass.bytes);
    state.wifi_pass.len = 0;
    state.wifi_pass_active = false;
    state.wifi_pass_shown = false;
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
