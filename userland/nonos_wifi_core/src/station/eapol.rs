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

use super::receive::{Rx, RxDrop};
use super::LinkStation;

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
        // Message 4 of a rekey goes out under the pairwise key in use; the
        // new one is taken only after it is framed.
        let pairwise_key = out.new_pairwise_key.then(|| sup.tk().try_into().ok()).flatten();
        if let Some((id, key, rsc)) = group_key {
            self.set_group_key(id, key, rsc);
        }
        let frame = out.reply.and_then(|r| self.eapol_frame(&r, protected));
        if let Some(tk) = pairwise_key {
            self.set_pairwise_key(tk);
        }
        let group_key = group_key.map(|(id, key, _)| (id, key));
        Rx::Handshake { frame, group_key, pairwise_key }
    }
}
