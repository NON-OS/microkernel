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

//! The two ways a management frame can end the link, and the unprotected one
//! that only asks the question.

use super::mgmt_rx::{leave_reason, MgmtDrop, MgmtRx, REASON_CLASS2, REASON_CLASS3};
use super::LinkStation;
use crate::pmf::bip_verify;

impl LinkStation {
    pub(super) fn leave(&mut self, reason: Option<u16>) -> Result<MgmtRx, MgmtDrop> {
        let reason = reason.ok_or(MgmtDrop::NotHandled)?;
        self.deassociate();
        Ok(MgmtRx::Left(reason))
    }

    // Group addressed: only a deauthentication or disassociation is acted on,
    // and only under the IGTK the link holds, with an IPN above its floor.
    pub(super) fn group_mgmt(&mut self, f: &[u8], sub: u8) -> Result<MgmtRx, MgmtDrop> {
        let reason = leave_reason(sub, &f[24..]).ok_or(MgmtDrop::NotHandled)?;
        let (igtk, id) = self.pmf.igtk.ok_or(MgmtDrop::Undecryptable)?;
        let mmie = bip_verify(f, &igtk).filter(|m| m.key_id == id);
        let mmie = mmie.ok_or(MgmtDrop::Undecryptable)?;
        if mmie.ipn <= self.pmf.ipn_floor {
            return Err(MgmtDrop::Replay);
        }
        self.pmf.ipn_floor = mmie.ipn;
        self.leave(Some(reason))
    }

    // Reason 6 or 7 is what a real AP that rebooted sends; anything else
    // unprotected is noise or a forgery.
    pub(super) fn unprotected_on_pmf(&mut self, sub: u8, body: &[u8]) -> Result<MgmtRx, MgmtDrop> {
        match leave_reason(sub, body) {
            Some(REASON_CLASS2 | REASON_CLASS3) if self.pmf.query.is_none() => {
                Ok(MgmtRx::SaQueryNeeded)
            }
            _ => Err(MgmtDrop::Unprotected),
        }
    }
}
