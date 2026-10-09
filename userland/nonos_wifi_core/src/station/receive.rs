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

//! The receive half of an associated link: what the access point sent, checked
//! before anything reaches the IP stack or the supplicant.
//!
//! A data frame is taken only from the BSS (FromDS, transmitter the BSSID) to
//! this station or a group address. Fragments and A-MSDUs are refused (this
//! station negotiates neither, and both are what the FragAttacks injections
//! ride on). On a protected link an unprotected frame is refused unless it is
//! EAPOL (a repeated message 3 arrives in the clear before the AP installs the
//! pairwise key); every protected frame is decrypted, by the chip (which
//! leaves the CCMP header and MIC in place) or here under the pairwise or
//! group key its key index names, and must pass the per-TID replay check.
//! EAPOL goes to the supplicant, which answers the group key handshake; the
//! rest is converted to Ethernet for the stack.

use alloc::vec::Vec;

use super::LinkStation;
use crate::dot11::ccmp::{decrypt, pn_and_key, view, CCMP_HDR_LEN, MIC_LEN};
use crate::dot11::header::{fc_subtype, fc_type, FC_FROM_DS, FC_PROTECTED, FC_TO_DS, TYPE_DATA};
use crate::frame::LLC_SNAP;

const ETHERTYPE_EAPOL: [u8; 2] = [0x88, 0x8E];
const FC_MORE_FRAGMENTS: u16 = 0x0400;
/// The A-MSDU Present bit of the QoS Control field.
const QOS_AMSDU: u8 = 0x80;
/// Data subtypes that carry no payload (Null, QoS Null and the CF ones).
const SUBTYPE_NO_DATA: u8 = 0x04;

/// Why a received frame was not delivered.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RxDrop {
    NotAssociated,
    /// Not a data frame with a payload, or shorter than its headers.
    Malformed,
    /// Not FromDS from the BSSID.
    NotFromBss,
    /// Addressed to another station.
    NotForUs,
    Fragmented,
    Amsdu,
    /// An unprotected frame that is not EAPOL on a protected link.
    Unprotected,
    /// No key to decrypt it under, or the MIC failed.
    Undecryptable,
    Replay,
    /// The body is not LLC/SNAP encapsulated.
    NotLlc,
    /// EAPOL arrived with no supplicant to answer it.
    NoSupplicant,
}

/// What a received data frame calls for.
pub enum Rx {
    /// An Ethernet frame for the stack.
    Ethernet(Vec<u8>),
    /// A key handshake message was handled: send `frame` (if any) to the AP,
    /// and install `group_key` (index, key) if the AP delivered a new one.
    Handshake { frame: Option<Vec<u8>>, group_key: Option<(u8, [u8; 16])> },
    Dropped(RxDrop),
}

/// A frame that passed the checks: its plaintext body (from the LLC/SNAP
/// header on), whether it arrived protected, and its Ethernet addresses.
struct Checked {
    body: Vec<u8>,
    protected: bool,
    da: [u8; 6],
    sa: [u8; 6],
}

impl LinkStation {
    /// Check and convert one received MPDU (FCS already removed).
    /// `hw_decrypted` says the chip decrypted and verified a protected frame
    /// (it leaves the CCMP header and MIC in place).
    pub fn receive(&mut self, mpdu: &[u8], hw_decrypted: bool) -> Rx {
        match self.check(mpdu, hw_decrypted) {
            Ok(c) => self.deliver(&c),
            Err(why) => Rx::Dropped(why),
        }
    }

    fn check(&mut self, mpdu: &[u8], hw: bool) -> Result<Checked, RxDrop> {
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
            decrypt(mpdu, &key.ok_or(RxDrop::Undecryptable)?).ok_or(RxDrop::Undecryptable)?
        };
        if !self.replay.accept(replay_key, v.tid, pn) {
            return Err(RxDrop::Replay);
        }
        Ok(Checked { body, protected: true, da, sa })
    }

    fn deliver(&mut self, c: &Checked) -> Rx {
        let body = &c.body;
        if body.len() < 8 || body[..6] != LLC_SNAP {
            return Rx::Dropped(RxDrop::NotLlc);
        }
        if body[6..8] == ETHERTYPE_EAPOL {
            return self.on_eapol(&body[8..], c.protected);
        }
        let mut eth = Vec::with_capacity(14 + body.len() - 8);
        eth.extend_from_slice(&c.da);
        eth.extend_from_slice(&c.sa);
        eth.extend_from_slice(&body[6..]);
        Rx::Ethernet(eth)
    }
}
