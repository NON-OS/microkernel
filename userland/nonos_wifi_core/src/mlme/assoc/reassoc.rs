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

//! Reassociation (IEEE Std 802.11-2020, 11.3.5.4). A station already
//! associated within an ESS moves to another AP of it with a Reassociation
//! Request naming the AP it leaves. Everything else in the join, the
//! authentication, the RSNE and the four-way handshake, is as for an
//! association.

use super::*;
use crate::dot11::mgmt::reassoc_request_elements;

impl Mlme {
    /// Make this join a move from `current_ap`, the AP the station holds.
    pub fn reassociate_from(&mut self, current_ap: [u8; 6]) {
        self.reassoc_from = Some(current_ap);
    }

    pub(super) fn request_frame(
        &self,
        out: &mut [u8],
        seq: u16,
        elements: &[&[u8]],
    ) -> Option<usize> {
        let (us, bss) = (self.our_mac, self.bssid);
        match self.reassoc_from {
            Some(from) => {
                reassoc_request_elements(out, us, bss, from, CAP_ESS_PRIVACY, seq, elements)
            }
            None => assoc_request_elements(out, us, bss, CAP_ESS_PRIVACY, seq, elements),
        }
    }
}
