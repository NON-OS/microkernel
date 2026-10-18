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

//! The station's SA Query on the link: started when an unprotected
//! deauthentication claims the AP lost the association, retried on the clock,
//! ending the link if the AP never answers. The caller supplies transaction
//! identifiers from its entropy, so the engine stays pure.

use alloc::vec::Vec;

use super::mgmt_rx::SaTick;
use super::LinkStation;
use crate::dot11::header::seq_control;
use crate::pmf::{sa_query_frame, SaPoll, SaQuery, SaQueryAction};

impl LinkStation {
    /// Start an SA Query; the first request to send. `None` when the link
    /// has no management frame protection or one is already running.
    pub fn sa_query_start(&mut self, now_ms: u64, txid: [u8; 2]) -> Option<Vec<u8>> {
        if !self.associated || !self.pmf.on || self.pmf.query.is_some() {
            return None;
        }
        self.pmf.query = Some(SaQuery::start(now_ms, txid));
        self.sa_query_tx(SaQueryAction::Request, txid)
    }

    /// Advance a running query to `now_ms`; `fresh` names a retry.
    pub fn sa_query_poll(&mut self, now_ms: u64, fresh: [u8; 2]) -> SaTick {
        let Some(q) = self.pmf.query.as_mut() else {
            return SaTick::Idle;
        };
        match q.poll(now_ms, fresh) {
            SaPoll::Wait => SaTick::Idle,
            SaPoll::Retry(txid) => {
                self.sa_query_tx(SaQueryAction::Request, txid).map_or(SaTick::Idle, SaTick::Send)
            }
            SaPoll::Expired => {
                self.deassociate();
                SaTick::Ended
            }
        }
    }

    // Frame an SA Query and protect it as data is protected.
    pub(super) fn sa_query_tx(&mut self, action: SaQueryAction, txid: [u8; 2]) -> Option<Vec<u8>> {
        let seq = self.seq;
        self.seq = self.seq.wrapping_add(1) & 0x0FFF;
        let plain = sa_query_frame(self.our_mac, self.bssid, seq_control(seq), action, txid);
        if self.pairwise_key().is_none() {
            return Some(plain);
        }
        self.protect_mgmt(&plain)
    }
}
