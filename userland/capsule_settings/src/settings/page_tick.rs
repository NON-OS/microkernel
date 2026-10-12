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

//! What a tick does for the page that is open. The Wi-Fi and Network pages
//! re-poll net_core, so a lease that binds a few seconds after connecting
//! shows its address without another scan. The Bluetooth page asks
//! driver.bt_usb0 again, which sends nothing unprompted (decision 7 of
//! letter E-25), so a question pairing asks is seen only by asking.

use crate::bluetooth::{panel::poll_ms, poll};

use super::section::Section;
use super::state::wifi_enter::refresh_wifi_status;
use super::state::State;

/// Whether the page changed and wants painting.
pub fn page_tick(state: &mut State) -> bool {
    match state.section {
        Section::Wifi | Section::Network => refresh_wifi_status(state),
        Section::Bluetooth => poll(&mut state.bt, false),
        _ => return false,
    }
    true
}

pub fn page_tick_ms(state: &State) -> i64 {
    match state.section {
        Section::Bluetooth => poll_ms(&state.bt),
        _ => 1000,
    }
}
