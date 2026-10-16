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

use crate::wifi::{ScanNetwork, WifiInterface};

use super::audio_output::AudioOutput;
use super::cache::FieldValue;
use super::edit_buffer::EditBuffer;
use super::machine_key::MachineKey;
use crate::settings::section::{Section, SECTION_COUNT};

use super::search::Search;
use super::state::{State, WifiConnect, WifiScan, FIELD_SLOTS, WIFI_MAX, WIFI_NET_MAX};
use super::status::Status;
use super::wifi::diag::WifiDiag;
use super::wifi::extra::WifiExtra;
use super::wifi::pass::WifiPass;

pub fn new() -> State {
    State {
        policy_port: 0,
        policy_ready: false,
        section: Section::General,
        cursor: [0; SECTION_COUNT],
        scroll_px: [0; SECTION_COUNT],
        search: Search::new(),
        values: [FieldValue::Unknown; FIELD_SLOTS],
        editing: false,
        edit: EditBuffer::empty(),
        status: Status::idle(),
        wifi_adapters: [WifiInterface::default(); WIFI_MAX],
        wifi_adapter_count: 0,
        wifi_cursor: 0,
        wifi_networks: [ScanNetwork::default(); WIFI_NET_MAX],
        wifi_network_count: 0,
        wifi_scan: WifiScan::Idle,
        wifi_diag: WifiDiag::new(),
        wifi_pass: WifiPass::new(),
        wifi_connect: WifiConnect::Idle,
        wifi: WifiExtra::new(),
        machine_key: MachineKey::Unasked,
        audio_output: AudioOutput::Unasked,
        access: crate::settings::access::AccessPage::default(),
        screens: crate::settings::screens::ScreenPage::default(),
        win_w: crate::settings::manifest::WIDTH,
        win_h: crate::settings::manifest::HEIGHT,
    }
}
