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

//! A request taken in: the authorization gate's ops, claims and virtual
//! ports answered; a claimant's slot made the parked one it stands for;
//! a binding weighed; then on to the controllers (`inner`).

use alloc::vec::Vec;

use super::mux::Mux;
use crate::protocol::{Request, OP_ENABLE_SLOT};
use crate::server::reply::vouch;

impl Mux {
    /// A request, a claimant's slot standing for the parked one it claimed.
    pub fn dispatch(&mut self, req: &Request, body: &[u8], tx: &mut [u8]) {
        if req.op == OP_ENABLE_SLOT {
            return self.enable_held(req, body, tx);
        }
        if self.auth_ops(req, body, tx) || self.claim_ops(req, body, tx) {
            return;
        }
        if self.virtual_ops(req, body, tx) {
            return;
        }
        // A claimant's slot stands for the parked one; the body is copied
        // only then.
        let own: Vec<u8>;
        let body = match body.first().map(|&s| self.virt.real(s)) {
            Some(real) if real != body[0] => {
                // driver.usb_hub0 enabled the parked slot; its claimant
                // drives it.
                vouch();
                own = [&[real][..], &body[1..]].concat();
                &own[..]
            }
            _ => body,
        };
        if !self.bind_refused(req, body, tx) {
            self.inner(req, body, tx)
        }
    }
}
