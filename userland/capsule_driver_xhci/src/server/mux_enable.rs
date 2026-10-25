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

//! Enable Slot with its holder kept: the slot id the reply carries is the
//! sender's until it gives it back (slots/holders.rs).

use super::mux::{status, Mux};
use crate::protocol::{Request, RESP_HDR_LEN, STATUS_LEN};
use crate::server::reply::{captured, send, sender};

impl Mux {
    pub(super) fn enable_held(&mut self, req: &Request, body: &[u8], tx: &mut [u8]) {
        let len = captured(|| self.inner(req, body, tx));
        let at = RESP_HDR_LEN + STATUS_LEN;
        if status(tx, len) == Some(0) && len > at {
            self.share.holders.set(tx[at], sender());
            // A new device on the slot is judged afresh before it is bound.
            self.share.auth.forget(tx[at]);
        }
        send(tx.as_ptr(), len);
    }
}
