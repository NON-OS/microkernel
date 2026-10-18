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

//! The pages that keep their own keys rather than walking the settings
//! rows: Wi-Fi's networks, and Bluetooth's devices and passkey.

use nonos_app_skeleton::EventOutcome;

use crate::settings::section::Section;
use crate::settings::state::State;

use super::on_event_bt::on_event_bt;
use super::on_event_wifi::on_event_wifi;

pub(super) fn on_section_key(state: &mut State, code: u32) -> Option<EventOutcome> {
    match state.section {
        Section::Wifi => Some(on_event_wifi(state, code)),
        Section::Bluetooth => Some(on_event_bt(state, code)),
        _ => None,
    }
}
