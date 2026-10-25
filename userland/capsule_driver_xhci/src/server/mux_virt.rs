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
//! Virtual ports (nonos_usb_virt) at the multiplexer, so they work with
//! one controller or several: OP_PARK from driver.usb_hub0 (mux_park.rs),
//! a claim on a virtual port by ADDRESS_DEVICE, the slots of parked
//! children and their claimants given back as the table says, and
//! PORT_STATUS with the virtual ports after the root ports (mux_list.rs).

use super::mux::Mux;
use crate::protocol::{
    Request, E_BUSY, E_INVAL, OP_ADDRESS_DEVICE, OP_DISABLE_SLOT, OP_PORT_STATUS,
};
use crate::server::error::reply_with_status;
use crate::server::handlers::address_reply::reply_ok;
use nonos_usb_virt::{is_virtual, OP_PARK};

impl Mux {
    /// True when the request was a virtual port's and is answered.
    pub(super) fn virtual_ops(&mut self, req: &Request, body: &[u8], tx: &mut [u8]) -> bool {
        match (req.op, body) {
            (OP_PARK, _) => self.park(req, body, tx),
            (OP_ADDRESS_DEVICE, &[s, port]) if is_virtual(port) => self.claim(req, s, port, tx),
            (OP_DISABLE_SLOT, &[s]) if self.virt.knows(s) => self.give_back(req, s, tx),
            (OP_PORT_STATUS, []) => self.list(req, tx),
            _ => return false,
        }
        true
    }

    fn claim(&mut self, req: &Request, s: u8, port: u8, tx: &mut [u8]) {
        // A virtual port nothing was ever parked on names no port at all.
        if !self.fresh(s) || self.virt.at(port).is_none() {
            return reply_with_status(tx, req, E_INVAL);
        }
        match self.virt.claim(port, s) {
            Some(p) => reply_ok(tx, req, s, port, p.speed, p.mps0),
            None => reply_with_status(tx, req, E_BUSY),
        }
    }
}
