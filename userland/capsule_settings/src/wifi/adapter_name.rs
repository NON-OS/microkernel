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

//! The name an adapter is listed under: a specific one for the Intel and
//! Realtek parts known here, else a generic vendor or Wi-Fi label, so an
//! unknown but valid card still lists.

/// A friendly name from vendor and device id. Known Intel and Realtek WiFi
/// parts get a specific label; any other WiFi-class card gets a generic vendor
/// label so it still lists.
pub fn adapter_label(vendor: u16, device: u16) -> &'static str {
    const INTEL: u16 = 0x8086;
    const REALTEK: u16 = 0x10EC;
    match (vendor, device) {
        (REALTEK, 0xC821) => "Realtek RTL8821CE",
        (INTEL, 0x2723 | 0x02F0 | 0x06F0 | 0x34F0 | 0x3DF0 | 0x4DF0 | 0x43F0) => {
            "Intel Wi-Fi 6 AX200"
        }
        (INTEL, 0x2725 | 0x2726 | 0x51F0 | 0x51F1 | 0x54F0 | 0x7AF0 | 0x7E40) => {
            "Intel Wi-Fi 6E AX210"
        }
        (INTEL, _) => "Intel Wi-Fi",
        (REALTEK, _) => "Realtek Wi-Fi",
        _ => "Wi-Fi adapter",
    }
}
