// NONOS Operating System (AGPL-3.0-or-later)

//! An EAPOL payload framed to the access point.

use alloc::vec::Vec;

use super::LinkStation;
use crate::dot11::data::build_data;

const ETHERTYPE_EAPOL: [u8; 2] = [0x88, 0x8E];

impl LinkStation {
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
