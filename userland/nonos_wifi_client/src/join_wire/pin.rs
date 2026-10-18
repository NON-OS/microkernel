/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

//! Whether a joined network is to be pinned to WPA3: it ran SAE (AKM 8, or 24
//! with an extended key), or message 3's Transition Disable KDE set bit 0.

use super::join_body::AKM_SAE;
use super::link::Link;

/// The AKM suite type of SAE with an extended key (SAE-EXT-KEY).
const AKM_SAE_EXT_KEY: u8 = 24;
/// Transition Disable bit 0: the network is WPA3-Personal only from now on.
const TD_WPA3_PERSONAL: u8 = 1 << 0;

impl Link {
    /// The association ran SAE, or the access point said in message 3 to
    /// disable the transition to WPA2: the saved network is to be pinned to
    /// WPA3, which only the person's Forget undoes.
    pub fn pins_wpa3(&self) -> bool {
        let sae = self.akm == AKM_SAE || self.akm == AKM_SAE_EXT_KEY;
        self.associated && (sae || self.transition_disable & TD_WPA3_PERSONAL != 0)
    }
    /// Associated with `ssid` in a way that pins it to WPA3.
    pub fn pins_wpa3_on(&self, ssid: &[u8]) -> bool {
        self.ssid() == ssid && self.pins_wpa3()
    }
}
