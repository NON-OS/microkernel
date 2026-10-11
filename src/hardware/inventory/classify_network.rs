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

use super::broadcom_ids::TG3_DEVICE_IDS;
use super::family::HardwareFamily;
use super::intel_ethernet_ids::{
    E1000E_DEVICE_IDS, E1000_DEVICE_IDS, IGB_DEVICE_IDS, IGC_DEVICE_IDS, IXGBE_DEVICE_IDS,
};
use super::realtek_ids::{RTL8139_DEVICE_ID, RTL8169_DEVICE_IDS, RTL8821CE_DEVICE_ID};

const SUBCLASS_OTHER: u8 = 0x80;

/// By device id, as the drivers bind: a part no table names is not taken
/// for one that does, so its row says no driver rather than the wrong one.
pub(super) fn classify_network(subclass: u8, vendor: u16, device: u16) -> HardwareFamily {
    match vendor {
        0x1af4 => HardwareFamily::NetworkVirtio,
        0x8086 if subclass == SUBCLASS_OTHER => HardwareFamily::NetworkIwlwifi,
        0x8086 => intel_ethernet(device),
        0x14e4 if TG3_DEVICE_IDS.contains(&device) => HardwareFamily::NetworkTg3,
        0x10ec if device == RTL8821CE_DEVICE_ID => HardwareFamily::NetworkRtl8821ce,
        // Realtek's other wireless parts (8822CE, 8852AE and the rest).
        0x10ec if subclass == SUBCLASS_OTHER => HardwareFamily::NetworkRealtekWifi,
        0x10ec if device == RTL8139_DEVICE_ID => HardwareFamily::NetworkRtl8139,
        0x10ec if RTL8169_DEVICE_IDS.contains(&device) => HardwareFamily::NetworkRtl8169,
        _ => HardwareFamily::Unknown,
    }
}

fn intel_ethernet(device: u16) -> HardwareFamily {
    if E1000_DEVICE_IDS.contains(&device) {
        HardwareFamily::NetworkE1000
    } else if E1000E_DEVICE_IDS.contains(&device) {
        HardwareFamily::NetworkE1000e
    } else if IGB_DEVICE_IDS.contains(&device) {
        HardwareFamily::NetworkIgb
    } else if IGC_DEVICE_IDS.contains(&device) {
        HardwareFamily::NetworkIgc
    } else if IXGBE_DEVICE_IDS.contains(&device) {
        HardwareFamily::NetworkIxgbe
    } else {
        HardwareFamily::Unknown
    }
}
