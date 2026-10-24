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

//! A Wi-Fi adapter as the panel lists it before any network scan (found by
//! `wifi_class`): a friendly name from its vendor and device id
//! (`adapter_name`), and the ids. The panel only names the adapter: it
//! neither selects nor drives one, so the record keeps no device handle.

use super::adapter_name::adapter_label;
use super::wifi_class::DeviceView;

/// The most an adapter name occupies, in bytes.
pub const NAME_MAX: usize = 32;

/// One discovered WiFi adapter as the panel lists it.
#[derive(Clone, Copy, Default)]
pub struct WifiInterface {
    name: [u8; NAME_MAX],
    name_len: usize,
    vendor: u16,
    device: u16,
}

impl WifiInterface {
    /// The friendly adapter name for display.
    pub fn name(&self) -> &[u8] {
        &self.name[..self.name_len]
    }

    /// The PCI vendor and device id.
    pub fn ids(&self) -> (u16, u16) {
        (self.vendor, self.device)
    }
}

pub(super) fn interface_of(d: &DeviceView) -> WifiInterface {
    let mut name = [0u8; NAME_MAX];
    let label = adapter_label(d.vendor, d.device);
    let len = label.len().min(NAME_MAX);
    name[..len].copy_from_slice(&label.as_bytes()[..len]);
    WifiInterface { name, name_len: len, vendor: d.vendor, device: d.device }
}
