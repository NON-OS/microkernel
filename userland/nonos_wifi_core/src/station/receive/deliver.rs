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

//! A checked frame handed on: to the stack as Ethernet, or to the supplicant.

use alloc::vec::Vec;

use super::check::ETHERTYPE_EAPOL;
use super::{Rx, RxDrop};
use crate::dot11::header::TYPE_MGMT;
use crate::frame::LLC_SNAP;
use crate::station::LinkStation;

/// A frame that passed the checks: its plaintext body (from the LLC/SNAP
/// header on), whether it arrived protected, and its Ethernet addresses.
pub(super) struct Checked {
    pub(super) body: Vec<u8>,
    pub(super) protected: bool,
    pub(super) da: [u8; 6],
    pub(super) sa: [u8; 6],
}

impl LinkStation {
    /// Check and convert one received MPDU (FCS already removed).
    /// `hw_decrypted` says the chip decrypted and verified a protected frame
    /// (it leaves the CCMP header and MIC in place). A management frame goes
    /// its own way; a data frame that passes counts as hearing the AP.
    pub fn receive(&mut self, mpdu: &[u8], hw_decrypted: bool) -> Rx {
        if mpdu.first().is_some_and(|b| (b >> 2) & 0x03 == TYPE_MGMT) {
            return self.on_mgmt(mpdu, hw_decrypted);
        }
        match self.check(mpdu, hw_decrypted) {
            Ok(c) => {
                self.watch.heard();
                self.deliver(&c)
            }
            Err(why) => Rx::Dropped(why),
        }
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
