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

use super::family::HardwareFamily;

pub(super) fn classify_network(subclass: u8, vendor: u16, device: u16) -> HardwareFamily {
    match vendor {
        0x1af4 => HardwareFamily::NetworkVirtio,
        0x8086 if subclass == 0x80 => HardwareFamily::NetworkIwlwifi,
        0x8086 => HardwareFamily::NetworkE1000,
        0x10ec if device == 0x8139 => HardwareFamily::NetworkRtl8139,
        0x10ec if device == 0xc821 => HardwareFamily::NetworkRtl8821ce,
        0x10ec => HardwareFamily::NetworkRtl8169,
        0x1022 if device == 0x2000 => HardwareFamily::NetworkPcnet,
        0x15ad if device == 0x07b0 => HardwareFamily::NetworkVmxnet3,
        0x1011 if device == 0x0009 || device == 0x0019 => HardwareFamily::NetworkTulip,
        _ => HardwareFamily::Unknown,
    }
}
