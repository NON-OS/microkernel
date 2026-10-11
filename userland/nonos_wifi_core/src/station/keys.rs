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

//! The link's keys and its view of the association: what the join handed
//! over, what a group rekey replaced, and what the driver may ask about.

use alloc::vec::Vec;

use super::{Ccmp, LinkStation};
use crate::wpa::supplicant::Supplicant;

impl LinkStation {
    /// Hand over the supplicant the join finished with, taking its group key
    /// and, with management frame protection, its integrity group key.
    pub fn set_supplicant(&mut self, sup: Supplicant) {
        self.pmf.take_igtk(&sup);
        if sup.gtk().len() == 16 {
            let mut key = [0u8; 16];
            key.copy_from_slice(sup.gtk());
            self.set_group_key(sup.gtk_id(), key, sup.gtk_rsc());
        }
        self.supplicant = Some(sup);
    }

    /// Take a group key for software decryption, with its Key RSC as the
    /// replay floor.
    pub fn set_group_key(&mut self, key_id: u8, key: [u8; 16], rsc: u64) {
        if let Some(slot) = self.group_keys.get_mut(key_id as usize) {
            *slot = Some(key);
            self.replay.set_group(key_id, rsc);
        }
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
}
