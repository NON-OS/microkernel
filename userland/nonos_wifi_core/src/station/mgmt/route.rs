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

//! Where a management frame goes: only one the access point sent, to this
//! station or to all, is looked at, and it counts as hearing the access point.

use crate::dot11::action::SUBTYPE_ACTION;
use crate::dot11::auth::{SUBTYPE_DEAUTH, SUBTYPE_DISASSOC};
use crate::dot11::header::{fc_subtype, MAC_HEADER_LEN};
use crate::station::{LinkStation, Rx, RxDrop};

/// The Order bit: an HT Control field follows Address 3, which no management
/// frame this station acts on carries.
const FC_ORDER: u16 = 0x8000;

impl LinkStation {
    pub(in crate::station) fn on_mgmt(&mut self, mpdu: &[u8], hw: bool) -> Rx {
        if !self.associated {
            return Rx::Dropped(RxDrop::NotAssociated);
        }
        if mpdu.len() < MAC_HEADER_LEN {
            return Rx::Dropped(RxDrop::Malformed);
        }
        let group = mpdu[4] & 0x01 != 0;
        if mpdu[10..16] != self.bssid || (!group && mpdu[4..10] != self.our_mac) {
            return Rx::Dropped(RxDrop::Management);
        }
        let fc = u16::from_le_bytes([mpdu[0], mpdu[1]]);
        if fc & FC_ORDER != 0 {
            return Rx::Dropped(RxDrop::Malformed);
        }
        self.watch.heard();
        match fc_subtype(fc) {
            SUBTYPE_DEAUTH | SUBTYPE_DISASSOC => self.on_leave(mpdu, hw, group),
            SUBTYPE_ACTION => self.on_action(mpdu, hw, group),
            _ => Rx::Dropped(RxDrop::Management),
        }
    }

    /// Whether management frame protection was negotiated for this link.
    pub(in crate::station) fn pmf(&self) -> bool {
        self.supplicant.as_ref().is_some_and(|s| s.pmf())
    }
}
