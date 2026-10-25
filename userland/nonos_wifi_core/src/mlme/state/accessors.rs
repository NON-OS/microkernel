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

//! The AKM the join runs, once a BSS was selected.

use super::*;

impl Mlme {
    /// The AKM the join runs, once a BSS was selected.
    pub fn akm(&self) -> Option<Akm> {
        self.selection.map(|s| s.akm)
    }

    pub fn channel(&self) -> u8 {
        self.channel
    }

    /// The station MAC and the joined AP's BSSID, needed by the data path to
    /// address encrypted frames. Meaningful once a BSS has been selected.
    pub fn our_mac(&self) -> [u8; 6] {
        self.our_mac
    }

    pub fn bssid(&self) -> [u8; 6] {
        self.bssid
    }

    pub(in crate::mlme) fn ssid(&self) -> &[u8] {
        &self.ssid[..self.ssid_len]
    }

    pub(in crate::mlme) fn secret(&self) -> &[u8] {
        &self.secret[..self.secret_len]
    }
}
