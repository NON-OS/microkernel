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

//! Nothing acts on an MPDU until it has passed fragment, protection, key
//! index and replay checks here.

use super::*;

impl LinkStation {
    pub(super) fn check(&mut self, mpdu: &[u8], hw: bool) -> Result<Checked, RxDrop> {
        if !self.associated {
            return Err(RxDrop::NotAssociated);
        }
        let v = view(mpdu).ok_or(RxDrop::Malformed)?;
        if fc_type(v.fc) != TYPE_DATA || fc_subtype(v.fc) & SUBTYPE_NO_DATA != 0 {
            return Err(RxDrop::Malformed);
        }
        if v.fc & (FC_TO_DS | FC_FROM_DS) != FC_FROM_DS || mpdu[10..16] != self.bssid {
            return Err(RxDrop::NotFromBss);
        }
        let group = mpdu[4] & 0x01 != 0;
        if !group && mpdu[4..10] != self.our_mac {
            return Err(RxDrop::NotForUs);
        }
        if v.fc & FC_MORE_FRAGMENTS != 0 || mpdu[22] & 0x0F != 0 {
            return Err(RxDrop::Fragmented);
        }
        if v.qos_at.is_some_and(|q| mpdu[q] & QOS_AMSDU != 0) {
            return Err(RxDrop::Amsdu);
        }
        let mut da = [0u8; 6];
        let mut sa = [0u8; 6];
        da.copy_from_slice(&mpdu[4..10]);
        sa.copy_from_slice(&mpdu[16..22]);
        // The AP relays a station's own broadcasts back to the whole BSS; one of
        // ours coming home is not news to the stack.
        if group && sa == self.our_mac {
            return Err(RxDrop::NotForUs);
        }
        if v.fc & FC_PROTECTED == 0 {
            let body = &mpdu[v.hdr_len..];
            if body.len() < 8 || body[..6] != LLC_SNAP || body[6..8] != ETHERTYPE_EAPOL {
                return Err(RxDrop::Unprotected);
            }
            return Ok(Checked { body: body.to_vec(), protected: false, da, sa });
        }
        let (pn, key_id) = pn_and_key(mpdu, v.hdr_len).ok_or(RxDrop::Malformed)?;
        let replay_key = group.then_some(key_id);
        let body = if hw {
            mpdu[v.hdr_len + CCMP_HDR_LEN..mpdu.len() - MIC_LEN].to_vec()
        } else {
            let key = if group { self.group_key(key_id) } else { self.pairwise_key() };
            open(mpdu, &key.ok_or(RxDrop::Undecryptable)?).ok_or(RxDrop::Undecryptable)?
        };
        if !self.replay.accept(replay_key, v.tid, pn) {
            return Err(RxDrop::Replay);
        }
        Ok(Checked { body, protected: true, da, sa })
    }
}
