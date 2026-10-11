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

//! Ethernet to 802.11 and back, under the link's protection.

use alloc::vec::Vec;

use super::{Ccmp, LinkStation};
use crate::dot11::data::{build_data, parse_data, protect, unprotect};

impl LinkStation {
    /// Frame an ethernet packet for transmission to the AP. For a hardware-CCMP
    /// chip this is the plaintext MPDU the radio will encrypt; for software CCMP
    /// it is the fully encrypted frame. Advances the sequence number, and for
    /// software CCMP the packet number. Returns `None` if the station is not
    /// associated or the ethernet frame is malformed.
    pub fn tx_frame(&mut self, eth: &[u8]) -> Option<Vec<u8>> {
        if !self.associated {
            return None;
        }
        let seq = self.seq;
        let mpdu = build_data(eth, self.our_mac, self.bssid, seq)?;
        // Only commit the counter once framing has succeeded.
        self.seq = self.seq.wrapping_add(1) & 0x0FFF;
        match &self.ccmp {
            Ccmp::Hardware => {
                // Pure-hardware CCMP hands the radio a plaintext frame; the engine
                // inserts the IV and ciphertext and sets the Protected bit itself.
                Some(mpdu)
            }
            Ccmp::Software { tk } | Ccmp::SoftwareTxHwRx { tk } => {
                // CCMP packet numbers start at 1; pre-increment from zero.
                let pn = self.pn.wrapping_add(1);
                let frame = protect(&mpdu, pn, tk)?;
                self.pn = pn;
                Some(frame)
            }
        }
    }

    /// Recover the ethernet frame from a received 802.11 data MPDU. For a
    /// hardware-CCMP chip the MPDU is already decrypted, so it is only parsed;
    /// for software CCMP it is decrypted first. Returns `None` if the frame is
    /// malformed, the MIC fails, or the body is not LLC/SNAP encapsulated.
    pub fn rx_frame(&self, mpdu: &[u8]) -> Option<Vec<u8>> {
        match &self.ccmp {
            Ccmp::Hardware | Ccmp::SoftwareTxHwRx { .. } => parse_data(mpdu),
            Ccmp::Software { tk } => parse_data(&unprotect(mpdu, tk)?),
        }
    }
}
