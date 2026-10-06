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

//! Key handshake messages on an associated link. The access point's group key
//! handshakes, and a message 3 it repeats because message 4 was lost, arrive
//! as EAPOL data frames after the join is over; the supplicant the join
//! finished with answers them here. A reply goes out protected under the
//! pairwise key when the message came protected, and in the clear when it did
//! not (the AP has not installed the pairwise key yet, so it could not read a
//! protected one). A new group key is taken for software decryption with its
//! Key RSC as the replay floor and handed to the driver to install.

use alloc::vec::Vec;

use super::receive::{Rx, RxDrop};
use super::LinkStation;
use crate::dot11::data::build_data;

const ETHERTYPE_EAPOL: [u8; 2] = [0x88, 0x8E];

impl LinkStation {
    pub(super) fn on_eapol(&mut self, eapol: &[u8], protected: bool) -> Rx {
        let Some(sup) = self.supplicant.as_mut() else {
            return Rx::Dropped(RxDrop::NoSupplicant);
        };
        let out = sup.step(eapol);
        let group_key = if out.new_group_key && sup.gtk().len() == 16 {
            let mut key = [0u8; 16];
            key.copy_from_slice(sup.gtk());
            Some((sup.gtk_id(), key, sup.gtk_rsc()))
        } else {
            None
        };
        if let Some((id, key, rsc)) = group_key {
            self.set_group_key(id, key, rsc);
        }
        let frame = out.reply.and_then(|r| self.eapol_frame(&r, protected));
        Rx::Handshake { frame, group_key: group_key.map(|(id, key, _)| (id, key)) }
    }

    /// Frame an EAPOL payload to the access point, protected under the
    /// pairwise key or in the clear.
    pub fn eapol_frame(&mut self, eapol: &[u8], protect: bool) -> Option<Vec<u8>> {
        let mut eth = Vec::with_capacity(14 + eapol.len());
        eth.extend_from_slice(&self.bssid);
        eth.extend_from_slice(&self.our_mac);
        eth.extend_from_slice(&ETHERTYPE_EAPOL);
        eth.extend_from_slice(eapol);
        if protect {
            return self.tx_frame(&eth);
        }
        let seq = self.seq;
        let frame = build_data(&eth, self.our_mac, self.bssid, seq)?;
        self.seq = self.seq.wrapping_add(1) & 0x0FFF;
        Some(frame)
    }
}
