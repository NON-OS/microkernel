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

//! What the association holds: its addresses, its keys, its state.

use alloc::vec::Vec;

use super::{Ccmp, LinkStation};
use crate::dot11::header::MacAddr;

impl LinkStation {
    /// The BSSID of the current association.
    pub fn bssid(&self) -> MacAddr {
        self.bssid
    }

    /// Protect a robust management frame to the AP (a deauthentication, with
    /// management frame protection on) under the pairwise key, advancing the
    /// packet number data frames share. `None` without a software key.
    pub fn protect_mgmt(&mut self, frame: &[u8]) -> Option<Vec<u8>> {
        let tk = self.pairwise_key()?;
        let pn = self.pn.wrapping_add(1);
        let out = crate::dot11::ccmp::encrypt(frame, pn, &tk)?;
        self.pn = pn;
        Some(out)
    }

    pub(super) fn pairwise_key(&self) -> Option<[u8; 16]> {
        match &self.ccmp {
            Ccmp::Hardware => None,
            Ccmp::Software { tk } | Ccmp::SoftwareTxHwRx { tk } => Some(*tk),
        }
    }

    pub(super) fn group_key(&self, key_id: u8) -> Option<[u8; 16]> {
        self.group_keys.get(key_id as usize).copied().flatten()
    }

    /// Whether the station is associated and carrying data.
    pub fn is_associated(&self) -> bool {
        self.associated
    }

    /// The station MAC address.
    pub fn mac(&self) -> MacAddr {
        self.our_mac
    }
}
