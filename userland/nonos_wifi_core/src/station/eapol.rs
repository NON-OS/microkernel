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
//! Key RSC as the replay floor and handed to the driver to install; a rekey's
//! pairwise key replaces the one in use once message 4 is framed.

use super::receive::{Rx, RxDrop};
use super::LinkStation;
use crate::rsn::TemporalKey;
use crate::vault::wipe;

impl LinkStation {
    pub(super) fn on_eapol(&mut self, eapol: &[u8], protected: bool) -> Rx {
        let Some(sup) = self.supplicant.as_mut() else {
            return Rx::Dropped(RxDrop::NoSupplicant);
        };
        let out = sup.step(eapol);
        // A group rekey under management frame protection carries a new IGTK.
        self.pmf.take_igtk(sup);
        let fresh = out.new_group_key.then(|| TemporalKey::new(sup.group_cipher(), sup.gtk()));
        let (id, rsc) = (sup.gtk_id(), sup.gtk_rsc());
        // Message 4 of a rekey goes out under the pairwise key in use; the
        // new one is taken only after it is framed.
        let mut tk = [0u8; 32];
        let n = if out.new_pairwise_key { sup.tk().len().min(32) } else { 0 };
        tk[..n].copy_from_slice(&sup.tk()[..n]);
        let fresh = fresh.flatten();
        // Only a CCMP-128 key goes to the chip's key cache; a GCMP-256 one
        // stays with the station, which decrypts group frames under it.
        let chip = match &fresh {
            Some(TemporalKey::Ccmp128(k)) => Some((id, *k)),
            _ => None,
        };
        // A key the station already holds in that slot, an AP handing back
        // one it rotated away from among them, is not installed again: not
        // here, and not in the chip's key cache (802.11-2020, 12.7.7.3).
        let taken = fresh.is_some_and(|key| self.set_group_key(id, key, rsc));
        let group_key = chip.filter(|_| taken);
        let frame = out.reply.and_then(|r| self.eapol_frame(&r, protected));
        if n != 0 {
            self.set_pairwise_key(&tk[..n]);
        }
        // Only a CCMP-128 key goes to the chip, as for the group key.
        let pairwise_key = tk[..n].try_into().ok();
        wipe(&mut tk);
        Rx::Handshake { frame, group_key, pairwise_key }
    }
}
