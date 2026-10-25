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

//! The address a driver joins a network under when it learns the network's
//! security from the beacon (`Security::of_beacon`), and one for a single
//! hunt or scan burst. The derivation is `Addresses::join`'s, the only one.

use super::bits::local_unicast;
use super::keys::Addresses;
use super::kind::Security;

/// Bytes of key the driver draws once a boot, into the vault itself
/// (`Addresses::for_this_boot`).
pub const KEY_LEN: usize = 32;

impl Addresses {
    /// The address to join `ssid` under `kind`. Never the BSSID: every
    /// access point of a network shares it.
    pub fn for_network(&self, ssid: &[u8], kind: Security) -> [u8; 6] {
        self.join(ssid, kind.octet())
    }
}

/// An address for one hunt or one scan burst, tied to no network and no
/// key: `random`, from the kernel's randomness, made local and unicast, so
/// nothing sent under it links to the station's other addresses.
pub fn one_off(random: &[u8; 6]) -> [u8; 6] {
    local_unicast(*random)
}
