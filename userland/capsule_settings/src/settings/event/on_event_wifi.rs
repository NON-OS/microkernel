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

use nonos_app_skeleton::EventOutcome;

use crate::settings::state::refresh_wifi::run_wifi_scan;
use crate::settings::state::wifi_join::{connect_selected, leave};
use crate::settings::state::wifi_remember::toggle_remember;
use crate::settings::state::wifi_saved::forget_selected;
use crate::settings::state::State;

use super::next_section::{next_section, prev_section};
use super::on_wifi_passphrase::on_passphrase_key;
use super::wifi_key::{wifi_key, WifiKey};
use super::wifi_radio_key::toggle_radio;

pub(super) fn on_event_wifi(state: &mut State, code: u32) -> EventOutcome {
    if state.wifi_pass_active {
        return on_passphrase_key(state, code);
    }
    let Some(key) = wifi_key(code) else { return EventOutcome::Idle };
    match key {
        WifiKey::Close => return EventOutcome::Close,
        WifiKey::NextSection => next_section(state),
        WifiKey::PrevSection => prev_section(state),
        WifiKey::Up => state.wifi_cursor = state.wifi_cursor.saturating_sub(1),
        WifiKey::Down => {
            if state.wifi_cursor + 1 < state.wifi_network_count + state.wifi.saved_count {
                state.wifi_cursor += 1;
            }
        }
        WifiKey::Scan => run_wifi_scan(state),
        WifiKey::Join => connect_selected(state),
        WifiKey::Leave => leave(state),
        WifiKey::Remember => toggle_remember(state),
        WifiKey::Forget => forget_selected(state),
        WifiKey::Radio => toggle_radio(state),
    }
    EventOutcome::Repaint
}

pub(super) fn repaint_after(state: &mut State, f: fn(&mut State)) -> EventOutcome {
    f(state);
    EventOutcome::Repaint
}
