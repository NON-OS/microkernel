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

//! The SA Query a deauthentication in the clear calls for, started by the
//! station itself from `receive`: its first request goes out at once, under
//! the station's own identifiers, and the reason is kept for the loss a
//! query left unanswered reports.

use super::{LinkStation, Rx, RxDrop};

impl LinkStation {
    // Start the SA Query a deauthentication with `reason` asked for.
    pub(super) fn ask(&mut self, reason: u16) -> Rx {
        let (now, txid) = (self.watch.clock, self.watch.txid());
        let Some(request) = self.sa_query_start(now.unwrap_or(0), txid) else {
            return Rx::Dropped(RxDrop::Unprotected);
        };
        self.watch.took_txid();
        self.watch.asked = Some(reason);
        self.watch.unstamped = now.is_none().then_some(txid);
        Rx::Send(request)
    }
}
