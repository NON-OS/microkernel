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

//! SA Query both ways. The station asks after a deauthentication in the clear
//! (`ask`), and answers the access point's own request. Both go protected
//! under the pairwise key, and only a protected query from the access point,
//! past the management replay counter, is taken.

use alloc::vec::Vec;

use crate::dot11::action::{parse_sa_query, sa_query_frame, SaQuery};
use crate::dot11::header::FC_PROTECTED;
use crate::station::{LinkStation, Rx, RxDrop};

impl LinkStation {
    pub(super) fn on_action(&mut self, mpdu: &[u8], hw: bool, group: bool) -> Rx {
        let protected = u16::from_le_bytes([mpdu[0], mpdu[1]]) & FC_PROTECTED != 0;
        if group || !protected || !self.pmf() {
            return Rx::Dropped(RxDrop::Management);
        }
        let body = match self.open_mgmt(mpdu, hw) {
            Ok(body) => body,
            Err(why) => return Rx::Dropped(why),
        };
        match parse_sa_query(&body) {
            Some(q) if !q.response => {
                self.sa_query(q.id, true).map_or(Rx::Dropped(RxDrop::Management), Rx::Send)
            }
            Some(q) => {
                self.watch.query.answered(q.id);
                Rx::Dropped(RxDrop::Management)
            }
            None => Rx::Dropped(RxDrop::Management),
        }
    }

    /// Start an SA Query about a deauthentication with `reason`: its first
    /// request, unless one already runs or there is no key to protect it.
    pub(super) fn ask(&mut self, reason: u16) -> Rx {
        if self.pairwise_key().is_none() {
            return Rx::Dropped(RxDrop::Unprotected);
        }
        match self.watch.query.start(self.watch.clock, reason) {
            Some(id) => self.sa_query(id, false).map_or(Rx::Dropped(RxDrop::Unprotected), Rx::Send),
            None => Rx::Dropped(RxDrop::Unprotected),
        }
    }

    /// The SA Query request or response `id`, protected under the pairwise key.
    pub(in crate::station) fn sa_query(&mut self, id: u16, response: bool) -> Option<Vec<u8>> {
        let frame = sa_query_frame(self.our_mac, self.bssid, self.seq, SaQuery { response, id });
        let out = self.protect_mgmt(&frame)?;
        self.seq = self.seq.wrapping_add(1) & 0x0FFF;
        Some(out)
    }
}
