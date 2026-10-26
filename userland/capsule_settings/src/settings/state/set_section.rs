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

use super::audio_output_probe::probe_audio_output;
use super::machine_key_probe::probe_machine_key;
use super::model::State;
use super::track_scroll::track_scroll;
use super::wifi::enter::enter_wifi;

/// Select a section. Entering Wi-Fi or Network enumerates adapters and re-reads
/// net_core, so both pages show the link as it is; neither scans, because a scan
/// blocks on the driver and would freeze the panel on navigation.
pub fn set_section(state: &mut State, section: Section) {
    state.section = section;
    state.editing = false;
    if matches!(section, Section::Wifi | Section::Network) {
        enter_wifi(state);
    }
    // Asked each time the page opens, so the row is what the TPM says now.
    if section == Section::Security {
        state.machine_key = probe_machine_key();
    }
    // Asked each time the page opens, so the row says whether this machine
    // can play sound, and why not, as things are now.
    if section == Section::Sound {
        state.audio_output = probe_audio_output();
    }
    // Asked each time the page opens, so its rows say what the display does.
    if section == Section::Accessibility {
        crate::settings::access::page::read(state);
    }
    if section == Section::Appearance {
        crate::settings::screens::read::read(state);
    }
    // Asked in full each time the page opens; the tick keeps it current.
    if section == Section::Bluetooth {
        crate::bluetooth::poll(&mut state.bt, true);
    }
    track_scroll(state);
}
