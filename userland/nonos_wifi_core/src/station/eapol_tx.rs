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
