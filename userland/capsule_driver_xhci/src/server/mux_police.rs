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
//! device nobody claimed only the driver that enabled its slot is served,
//! on a shared one only a claimant, and on a pipe only the one that
//! configured it (slots/gate.rs).

use nonos_libc::mk_pid_alive;

use super::mux::Mux;
use crate::protocol::Request;
use crate::server::error::reply_with_status;
use crate::server::reply::vouch;
use crate::slots::gate;

impl Mux {
    /// True when the request was refused here.
    pub(super) fn police(&mut self, req: &Request, body: &[u8], pid: u32, tx: &mut [u8]) -> bool {
        match gate(&mut self.share, req.op, body, pid, |p| mk_pid_alive(p)) {
            Ok(()) => {
                // A claimant of a shared device acts on a slot another
                // driver enabled; the controller takes it on this word.
                if body.first().is_some_and(|&s| self.share.claims.holds(s, pid)) {
                    vouch();
                }
                false
            }
            Err(e) => {
                reply_with_status(tx, req, e);
                true
            }
        }
    }
}
