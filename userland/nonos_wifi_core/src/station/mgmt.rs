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

//! Management frames on an associated link. Without management frame
//! protection the AP's deauthentication or disassociation ends the link as it
//! always has. With it, only a protected one does: under the pairwise key when
//! addressed to this station, under BIP when group addressed. An unprotected
//! one saying the AP lost the association (reason 6 or 7) asks for an SA
//! Query instead, since anyone can send it; every other one is dropped. Any
//! frame the AP sends, to this station or to all, counts as hearing it, and
//! one with the Order bit set (an HT Control field after Address 3, which no
//! frame the link acts on carries) is refused as malformed.

use super::mgmt_rx::{leave_reason, MgmtDrop, MgmtRx};
use super::LinkStation;
use crate::dot11::header::{
    fc_subtype, fc_type, FC_PROTECTED, MAC_HEADER_LEN, SUBTYPE_BEACON, TYPE_MGMT,
};

/// The Order bit of the frame control field.
const FC_ORDER: u16 = 0x8000;

impl LinkStation {
    /// Check and act on one received management frame (FCS removed).
    /// `hw_decrypted` says the chip decrypted a protected frame, leaving the
    /// CCMP header and MIC as received, as for data frames.
    pub fn receive_mgmt(&mut self, mpdu: &[u8], hw_decrypted: bool) -> MgmtRx {
        self.mgmt_checked(mpdu, hw_decrypted).unwrap_or_else(MgmtRx::Dropped)
    }

    fn mgmt_checked(&mut self, f: &[u8], hw: bool) -> Result<MgmtRx, MgmtDrop> {
        if !self.associated {
            return Err(MgmtDrop::NotAssociated);
        }
        let fc = u16::from_le_bytes([*f.first().unwrap_or(&0), *f.get(1).unwrap_or(&0)]);
        if f.len() < MAC_HEADER_LEN || fc_type(fc) != TYPE_MGMT || fc & FC_ORDER != 0 {
            return Err(MgmtDrop::Malformed);
        }
        let group = f[4] & 0x01 != 0;
        if f[10..16] != self.bssid
            || f[16..22] != self.bssid
            || (!group && f[4..10] != self.our_mac)
        {
            return Err(MgmtDrop::NotOurs);
        }
        let sub = fc_subtype(fc);
        if !(sub == SUBTYPE_BEACON && self.pmf.bigtk.is_some()) {
            self.watch.heard();
        }
        match (self.pmf.on, group, fc & FC_PROTECTED != 0) {
            (false, _, false) => self.leave(leave_reason(sub, &f[MAC_HEADER_LEN..])),
            (false, _, true) => Err(MgmtDrop::NotHandled),
            (true, true, _) => self.group_mgmt(f, sub),
            (true, false, false) => self.unprotected_on_pmf(sub, &f[MAC_HEADER_LEN..]),
            (true, false, true) => {
                let body = self.open_mgmt(f, hw)?;
                self.protected_mgmt(sub, &body)
            }
        }
    }
}
