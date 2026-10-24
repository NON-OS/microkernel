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

use super::cipher::seal;
use super::{Ccmp, LinkStation};
use crate::rsn::TemporalKey;
use crate::wpa::supplicant::Supplicant;

impl LinkStation {
    /// Hand over the supplicant the join finished with, taking its group key
    /// and, with management frame protection, its integrity group key.
    pub fn set_supplicant(&mut self, sup: Supplicant) {
        self.pmf.take_igtk(&sup);
        if let Some(key) = TemporalKey::new(sup.group_cipher(), sup.gtk()) {
            self.set_group_key(sup.gtk_id(), key, sup.gtk_rsc());
        }
        self.supplicant = Some(sup);
    }

    /// Take a group key for software decryption, with its Key RSC as the
    /// replay floor. The key its slot already holds keeps its counter: an AP
    /// that hands back a key it rotated away from, or a handshake replayed,
    /// never resets a key in use to an older floor. Returns whether it was taken:
    /// a key the slot holds is not installed again (802.11-2020, 12.7.7.3).
    pub fn set_group_key(&mut self, key_id: u8, key: TemporalKey, rsc: u64) -> bool {
        let Some(slot) = self.group_keys.get_mut(key_id as usize) else {
            return false;
        };
        if slot.as_ref() == Some(&key) {
            return false;
        }
        *slot = Some(key);
        self.replay.set_group(key_id, rsc);
        true
    }

    /// Protect a robust management frame to the AP (a deauthentication, with
    /// management frame protection on) under the pairwise key, advancing the
    /// packet number data frames share. `None` without a software key.
    pub fn protect_mgmt(&mut self, frame: &[u8]) -> Option<alloc::vec::Vec<u8>> {
        let tk = self.pairwise_key()?;
        let pn = self.pn.wrapping_add(1);
        let out = seal(frame, pn, &tk)?;
        self.pn = pn;
        Some(out)
    }

    pub(super) fn pairwise_key(&self) -> Option<TemporalKey> {
        match &self.ccmp {
            Ccmp::Hardware => None,
            Ccmp::Software { tk } | Ccmp::SoftwareTxHwRx { tk } => Some(TemporalKey::Ccmp128(*tk)),
            Ccmp::Gcmp256 { tk } => Some(TemporalKey::Gcmp256(*tk)),
        }
    }

    pub(super) fn group_key(&self, key_id: u8) -> Option<TemporalKey> {
        self.group_keys.get(key_id as usize)?.as_ref().map(TemporalKey::again)
    }
}
