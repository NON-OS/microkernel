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

//! What the current association installed.

use super::consts::PAIRWISE_KEY_ID;

/// What the current association installed, so a disconnect clears exactly it,
/// and the network's name and security, which the link op reports.
#[derive(Clone, Copy)]
pub(in crate::serve) struct Session {
    pub(in crate::serve) bssid: [u8; 6],
    pub(super) key_id: u8,
    pub(super) gtk_id: u8,
    pub(in crate::serve) ssid: [u8; 32],
    pub(in crate::serve) ssid_len: u8,
    /// The AKM suite type that ran (2 WPA2-PSK, 6 PSK-SHA256, 8 WPA3-SAE).
    pub(in crate::serve) akm: u8,
}

impl Session {
    /// A session just joined, its pairwise key in `PAIRWISE_KEY_ID`.
    pub(super) fn new(bssid: [u8; 6], gtk_id: u8, ssid: &[u8], akm: u8) -> Self {
        let (mut name, len) = ([0u8; 32], ssid.len().min(32));
        name[..len].copy_from_slice(&ssid[..len]);
        Session { bssid, key_id: PAIRWISE_KEY_ID, gtk_id, ssid: name, ssid_len: len as u8, akm }
    }

    /// Move the group key slot after a rekey the data path installed.
    pub(in crate::serve) fn set_gtk_id(&mut self, id: u8) {
        self.gtk_id = id;
    }
}
