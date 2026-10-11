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
//! Query instead, since anyone can send it; every other one is dropped.

use super::mgmt_rx::{leave_reason, MgmtDrop, MgmtRx};
use super::LinkStation;
use crate::dot11::header::{fc_subtype, fc_type, FC_PROTECTED, MAC_HEADER_LEN, TYPE_MGMT};

impl LinkStation {
    /// Check and act on one received management frame (FCS removed).
    /// `hw_decrypted` says the chip decrypted and verified a protected frame,
    /// leaving the CCMP header and MIC in place, as for data frames.
    pub fn receive_mgmt(&mut self, mpdu: &[u8], hw_decrypted: bool) -> MgmtRx {
        self.mgmt_checked(mpdu, hw_decrypted).unwrap_or_else(MgmtRx::Dropped)
    }

    fn mgmt_checked(&mut self, f: &[u8], hw: bool) -> Result<MgmtRx, MgmtDrop> {
        if !self.associated {
            return Err(MgmtDrop::NotAssociated);
        }
        let fc = u16::from_le_bytes([*f.first().unwrap_or(&0), *f.get(1).unwrap_or(&0)]);
        if f.len() < MAC_HEADER_LEN || fc_type(fc) != TYPE_MGMT {
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
