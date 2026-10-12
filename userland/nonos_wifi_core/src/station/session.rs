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

//! The keys an association takes after the join, and how it ends.

use super::{LinkStation, Loss};
use crate::wpa::supplicant::Supplicant;

impl LinkStation {
    /// Hand over the supplicant the join finished with, taking its group key.
    pub fn set_supplicant(&mut self, sup: Supplicant) {
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

    /// Drop the association; the station stops framing until it associates
    /// again, and forgets the keys and the supplicant.
    pub fn deassociate(&mut self) {
        self.associated = false;
        self.group_keys = [None; 4];
        self.supplicant = None;
    }

    /// End the link for `why`.
    pub(super) fn lose(&mut self, why: Loss) -> Loss {
        self.deassociate();
        why
    }
}
