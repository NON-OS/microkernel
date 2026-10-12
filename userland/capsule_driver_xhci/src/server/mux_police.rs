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
//! A request on a slot weighed before it reaches the controller: on a
//! shared device only a claimant is served, and on a pipe only the one
//! that configured it (slots/owners.rs). Every other device is served as
//! before.

use super::mux::Mux;
use crate::protocol::Request;
use crate::server::error::reply_with_status;
use crate::slots::{use_of, Asker};

impl Mux {
    /// True when the request was refused here.
    pub(super) fn police(&mut self, req: &Request, body: &[u8], pid: u32, tx: &mut [u8]) -> bool {
        let Some((slot, u)) = use_of(req.op, body) else { return false };
        let c = &self.share.claims;
        let who = Asker { pid, shared: c.any(slot), holds: c.holds(slot, pid) };
        match self.share.owners.admit(slot, u, who) {
            Ok(()) => false,
            Err(e) => {
                reply_with_status(tx, req, e);
                true
            }
        }
    }
}
