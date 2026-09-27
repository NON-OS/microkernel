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

use crate::settings::section::Section;

use super::state::State;
use super::track_scroll::track_scroll;
use super::wifi_enter::enter_wifi;

/// Select a section. Entering Wi-Fi or Network enumerates adapters and re-reads
/// net_core, so both pages show the link as it is; neither scans, because a scan
/// blocks on the driver and would freeze the panel on navigation.
pub fn set_section(state: &mut State, section: Section) {
    state.section = section;
    state.editing = false;
    if matches!(section, Section::Wifi | Section::Network) {
        enter_wifi(state);
    }
    track_scroll(state);
}
