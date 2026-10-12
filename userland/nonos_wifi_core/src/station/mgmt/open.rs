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

//! The two ways an access point protects a management frame. One sent to this
//! station is CCMP under the pairwise key, as data is, but checked against a
//! replay counter of its own (IEEE Std 802.11-2020, 12.5.3.4.4). One sent to
//! all carries BIP under the IGTK message 3 delivered, and its IPN must pass
//! the last one taken under that key, starting from the one message 3 named.

use alloc::vec::Vec;

use crate::dot11::bip::bip_open;
use crate::dot11::ccmp::{decrypt, pn_and_key, CCMP_HDR_LEN, MIC_LEN};
use crate::dot11::header::MAC_HEADER_LEN;
use crate::station::{LinkStation, RxDrop};

impl LinkStation {
    /// The plaintext body of a protected management frame to this station,
    /// decrypted by the chip (`hw`) or here, once past the replay counter.
    pub(super) fn open_mgmt(&mut self, mpdu: &[u8], hw: bool) -> Result<Vec<u8>, RxDrop> {
        let (pn, _) = pn_and_key(mpdu, MAC_HEADER_LEN).ok_or(RxDrop::Malformed)?;
        let body = if hw {
            mpdu[MAC_HEADER_LEN + CCMP_HDR_LEN..mpdu.len() - MIC_LEN].to_vec()
        } else {
            let tk = self.pairwise_key().ok_or(RxDrop::Undecryptable)?;
            decrypt(mpdu, &tk).ok_or(RxDrop::Undecryptable)?
        };
        if pn <= self.watch.mgmt_pn {
            return Err(RxDrop::Replay);
        }
        self.watch.mgmt_pn = pn;
        Ok(body)
    }

    /// Whether a frame sent to all holds under BIP: its MIC under the IGTK it
    /// names, and an IPN past the last one taken.
    pub(super) fn bip(&mut self, mpdu: &[u8]) -> bool {
        let Some((key, id, ipn)) = self.supplicant.as_ref().and_then(|s| s.igtk()) else {
            return false;
        };
        let (key, start) = (*key, le48(&ipn));
        let Some(m) = bip_open(mpdu, &key) else {
            return false;
        };
        let floor = match self.watch.ipn {
            Some((k, last)) if k == id => last,
            _ => start,
        };
        if m.key_id != id || m.ipn <= floor {
            return false;
        }
        self.watch.ipn = Some((id, m.ipn));
        true
    }
}

fn le48(b: &[u8; 6]) -> u64 {
    let mut w = [0u8; 8];
    w[..6].copy_from_slice(b);
    u64::from_le_bytes(w)
}
