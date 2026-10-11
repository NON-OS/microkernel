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

//! A protected data frame opened under the key it names.

use alloc::vec::Vec;

use super::RxDrop;
use crate::dot11::ccmp::{decrypt, pn_and_key, FrameView, CCMP_HDR_LEN, MIC_LEN};
use crate::station::LinkStation;

impl LinkStation {
    // A protected frame's plaintext, decrypted by the chip or here under the
    // pairwise or group key its key index names, once it passes the replay
    // check for its key and TID.
    pub(super) fn open(
        &mut self,
        mpdu: &[u8],
        v: &FrameView,
        hw: bool,
        group: bool,
    ) -> Result<Vec<u8>, RxDrop> {
        let (pn, key_id) = pn_and_key(mpdu, v.hdr_len).ok_or(RxDrop::Malformed)?;
        let replay_key = group.then_some(key_id);
        let body = if hw {
            mpdu[v.hdr_len + CCMP_HDR_LEN..mpdu.len() - MIC_LEN].to_vec()
        } else {
            let key = if group { self.group_key(key_id) } else { self.pairwise_key() };
            decrypt(mpdu, &key.ok_or(RxDrop::Undecryptable)?).ok_or(RxDrop::Undecryptable)?
        };
        if !self.replay.accept(replay_key, v.tid, pn) {
            return Err(RxDrop::Replay);
        }
        Ok(body)
    }
}
