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

//! The shared `KeyStore` over the chip's CAM.

use nonos_wifi_core::key::KeyStore;

use super::keys::{RtlKeys, PAIRWISE_BASE};
use crate::regs::Mmio;
use crate::sec::{clear_cam, write_cam, Key, CAM_AES};

impl<M: Mmio> KeyStore for RtlKeys<M> {
    fn install_ptk(&mut self, key: &[u8; 16], key_id: u8, peer: &[u8; 6]) -> bool {
        let Some(i) = self.slot_for(peer) else {
            return false;
        };
        let entry = Key { key: *key, mac: *peer, key_idx: key_id, pairwise: true };
        write_cam(&self.mmio, PAIRWISE_BASE + i as u8, CAM_AES, &entry);
        self.pairwise[i] = Some(*peer);
        true
    }

    fn install_gtk(&mut self, key: &[u8; 16], key_id: u8) -> bool {
        // Group keys map directly onto the reserved entries 0..3.
        if key_id >= PAIRWISE_BASE {
            return false;
        }
        let entry = Key { key: *key, mac: [0xFF; 6], key_idx: key_id, pairwise: false };
        write_cam(&self.mmio, key_id, CAM_AES, &entry);
        true
    }

    fn remove_key(&mut self, key_id: u8, peer: Option<&[u8; 6]>) {
        match peer {
            Some(p) => {
                if let Some(i) = self.pairwise.iter().position(|q| q.as_ref() == Some(p)) {
                    clear_cam(&self.mmio, PAIRWISE_BASE + i as u8);
                    self.pairwise[i] = None;
                }
            }
            None => clear_cam(&self.mmio, key_id),
        }
    }
}
