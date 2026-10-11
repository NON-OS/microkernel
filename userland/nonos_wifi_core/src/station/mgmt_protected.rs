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

//! A protected management frame addressed to this station: opened under the
//! pairwise key (or taken as the chip decrypted it), checked against its own
//! replay floor, then acted on: a deauthentication or disassociation ends the
//! link, an SA Query request is answered, a response to the station's own
//! query settles it.

use alloc::vec::Vec;

use super::mgmt_rx::{leave_reason, MgmtDrop, MgmtRx};
use super::LinkStation;
use crate::dot11::ccmp::{decrypt, pn_and_key, CCMP_HDR_LEN, MIC_LEN};
use crate::dot11::header::MAC_HEADER_LEN;
use crate::pmf::{parse_sa_query, SaQueryAction, SUBTYPE_ACTION};

impl LinkStation {
    pub(super) fn open_mgmt(&mut self, f: &[u8], hw: bool) -> Result<Vec<u8>, MgmtDrop> {
        let (pn, _) = pn_and_key(f, MAC_HEADER_LEN).ok_or(MgmtDrop::Malformed)?;
        let body = if hw {
            let end = f.len().checked_sub(MIC_LEN).ok_or(MgmtDrop::Malformed)?;
            f.get(MAC_HEADER_LEN + CCMP_HDR_LEN..end).ok_or(MgmtDrop::Malformed)?.to_vec()
        } else {
            let tk = self.pairwise_key().ok_or(MgmtDrop::Undecryptable)?;
            decrypt(f, &tk).ok_or(MgmtDrop::Undecryptable)?
        };
        if pn <= self.pmf.mgmt_pn {
            return Err(MgmtDrop::Replay);
        }
        self.pmf.mgmt_pn = pn;
        Ok(body)
    }

    pub(super) fn protected_mgmt(&mut self, sub: u8, body: &[u8]) -> Result<MgmtRx, MgmtDrop> {
        if let Some(reason) = leave_reason(sub, body) {
            return self.leave(Some(reason));
        }
        if sub != SUBTYPE_ACTION {
            return Err(MgmtDrop::NotHandled);
        }
        match parse_sa_query(body).ok_or(MgmtDrop::NotHandled)? {
            (SaQueryAction::Request, txid) => {
                let reply = self.sa_query_tx(SaQueryAction::Response, txid);
                reply.map(MgmtRx::Reply).ok_or(MgmtDrop::Undecryptable)
            }
            (SaQueryAction::Response, txid) if self.pmf.query.is_some_and(|q| q.answers(txid)) => {
                self.pmf.query = None;
                Ok(MgmtRx::Confirmed)
            }
            (SaQueryAction::Response, _) => Err(MgmtDrop::NotHandled),
        }
    }
}
