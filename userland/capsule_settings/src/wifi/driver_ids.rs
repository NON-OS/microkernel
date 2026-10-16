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

//! Which Wi-Fi chips this build ships a driver for, so the panel can say a
//! card has none rather than that one failed to start.

/// Whether this build ships a driver that takes a Wi-Fi chip: the RTL8821CE
/// driver for 10ec:c821, and the iwlwifi driver for the Intel ids it matches
/// (`capsule_driver_iwlwifi/src/firmware/family.rs` `family_for_device`). The
/// kernel spawns iwlwifi for every Intel wireless function, but the driver
/// leaves at once on an id outside that list (0x24F7 to 0x24FA and 0x24FC are
/// no adapter's), so such a chip has no driver, not one that failed to start.
/// Anything else (MediaTek MT7921/7922, Broadcom, Qualcomm ath10k/ath11k,
/// other Realtek parts) has none either.
pub fn has_driver(vendor: u16, device: u16) -> bool {
    match vendor {
        0x10EC => device == 0xC821,
        0x8086 => matches!(
            device,
            0x08B1..=0x08B4
                | 0x095A
                | 0x095B
                | 0x3165
                | 0x3166
                | 0x24FB
                | 0x24F3..=0x24F6
                | 0x24FD
                | 0x2526
                | 0x9DF0
                | 0xA370
                | 0x31DC
                | 0x30DC
                | 0x271B
                | 0x271C
                | 0x2723
                | 0x34F0
                | 0x3DF0
                | 0x4DF0
                | 0x02F0
                | 0x06F0
                | 0x43F0
                | 0xA0F0
                | 0x2725
                | 0x2729
                | 0x51F0
                | 0x51F1
                | 0x54F0
                | 0xA74F
                | 0x272F
                | 0x7A70
                | 0x7AF0
                | 0x7F70
                | 0x7E40
        ),
        _ => false,
    }
}
