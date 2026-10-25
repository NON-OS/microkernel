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

use crate::settings::section::{Section, SECTION_COUNT};
use crate::wifi::{ScanNetwork, WifiInterface};

use super::audio_output::AudioOutput;
use super::cache::FieldValue;
use super::edit_buffer::EditBuffer;
use super::machine_key::MachineKey;
use super::search::Search;
use super::status::Status;
use super::wifi::diag::WifiDiag;
use super::wifi::extra::WifiExtra;
pub use super::wifi::kinds::{WifiConnect, WifiScan};
use super::wifi::pass::WifiPass;

pub use super::limits::{FIELD_SLOTS, WIFI_MAX, WIFI_NET_MAX};

pub struct State {
    pub policy_port: u32,
    pub policy_ready: bool,
    pub section: Section,
    pub cursor: [usize; SECTION_COUNT],
    /// Pixel scroll per section: cards differ in height, so no row index.
    pub scroll_px: [u32; SECTION_COUNT],
    /// The titlebar search: its query, focus, result cursor and scroll.
    pub search: Search,
    pub values: [FieldValue; FIELD_SLOTS],
    pub editing: bool,
    pub edit: EditBuffer,
    pub status: Status,
    pub wifi_adapters: [WifiInterface; WIFI_MAX],
    pub wifi_adapter_count: usize,
    pub wifi_cursor: usize,
    /// The networks the last scan found, and how many are valid.
    pub wifi_networks: [ScanNetwork; WIFI_NET_MAX],
    pub wifi_network_count: usize,
    /// Whether a scan has run and, if so, how it resolved.
    pub wifi_scan: WifiScan,
    /// What the driver and net_core last said, for the panel to diagnose.
    pub wifi_diag: WifiDiag,
    /// The passphrase editor for a secured network.
    pub wifi_pass: WifiPass,
    /// The outcome of the last connection attempt.
    pub wifi_connect: WifiConnect,
    /// The driver found, its link, and the saved networks by name.
    pub wifi: WifiExtra,
    /// What the kernel said the last time Security asked for the machine key.
    pub machine_key: MachineKey,
    /// What audio.server said the last time the Sound page asked.
    pub audio_output: AudioOutput,
    /// The Accessibility page's display, and the displays' arrangement, as
    /// the compositor holds them; what driver.bt_usb0 last said.
    pub access: crate::settings::access::AccessPage,
    pub screens: crate::settings::screens::ScreenPage,
    pub bt: crate::bluetooth::Panel,
    /// Window size as of the last paint: the window is resizable, so layout
    /// and hit tests read this, never the manifest's starting size.
    pub win_w: u32,
    pub win_h: u32,
}
